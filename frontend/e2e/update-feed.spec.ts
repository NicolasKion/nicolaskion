// The tournament-tool update feed: installed desktop clients have its URL
// compiled in, so it must keep answering the same bytes, types, ranges
// and validators. Caddy serves it from disk (tests/fixtures/update-feed in
// the e2e stack, /srv/nicolaskion/update-feed in production).
import { readFileSync } from 'node:fs';
import { expect, test } from '@playwright/test';

const FEED = '/storage/tt-8d20886697466b8297bb6d6e1735fa53';

function fixture(name: string): Buffer {
  return readFileSync(new URL(`../../tests/fixtures/update-feed/${name}`, import.meta.url));
}

const FILES = [
  ['latest.json', 'application/json'],
  ['latest.json.sig', 'application/octet-stream'],
  ['update-v0.3.4-linux-x86_64.gz', 'application/octet-stream'],
  ['update-v0.3.4-macos-universal.app.tar.gz', 'application/octet-stream'],
  ['update-v0.3.4-windows-x86_64.exe.gz', 'application/octet-stream'],
] as const;

test('serves every file byte for byte with its type', async ({ request }) => {
  for (const [name, type] of FILES) {
    const response = await request.get(`${FEED}/${name}`, {
      headers: { 'Accept-Encoding': 'gzip, br, zstd' },
    });
    expect(response.status(), name).toBe(200);
    expect(response.headers()['content-type'].split(';')[0], name).toBe(type);
    expect(response.headers()['content-encoding'], name).toBeUndefined();
    expect(response.headers()['accept-ranges'], name).toBe('bytes');
    expect(response.headers()['last-modified'], name).toBeTruthy();
    expect(Number(response.headers()['content-length']), name).toBe(fixture(name).length);
    expect(Buffer.compare(await response.body(), fixture(name)), name).toBe(0);
  }
});

test('answers range requests', async ({ request }) => {
  const name = 'update-v0.3.4-linux-x86_64.gz';
  const total = fixture(name).length;
  const response = await request.get(`${FEED}/${name}`, { headers: { Range: 'bytes=100-1123' } });
  expect(response.status()).toBe(206);
  expect(response.headers()['content-range']).toBe(`bytes 100-1123/${total}`);
  expect(Buffer.compare(await response.body(), fixture(name).subarray(100, 1124))).toBe(0);

  const tail = await request.get(`${FEED}/${name}`, { headers: { Range: `bytes=${total - 10}-` } });
  expect(tail.status()).toBe(206);
  expect(Buffer.compare(await tail.body(), fixture(name).subarray(total - 10))).toBe(0);
});

test('answers conditional requests with 304', async ({ request }) => {
  const first = await request.get(`${FEED}/latest.json`);
  const etag = first.headers()['etag'];
  expect(etag).toBeTruthy();
  const again = await request.get(`${FEED}/latest.json`, { headers: { 'If-None-Match': etag } });
  expect(again.status()).toBe(304);
  const since = await request.get(`${FEED}/latest.json`, {
    headers: { 'If-Modified-Since': first.headers()['last-modified'] },
  });
  expect(since.status()).toBe(304);
});

test('answers HEAD like GET without a body', async ({ request }) => {
  const response = await request.head(`${FEED}/latest.json.sig`);
  expect(response.status()).toBe(200);
  expect(Number(response.headers()['content-length'])).toBe(fixture('latest.json.sig').length);
});

test('answers 404 for files that are not there', async ({ request }) => {
  for (const path of [`${FEED}/missing.json`, `${FEED}/`, `${FEED}/latest.json/x`]) {
    expect((await request.get(path)).status(), path).not.toBe(200);
  }
  expect((await request.get(`${FEED}/missing.json`)).status()).toBe(404);
});
