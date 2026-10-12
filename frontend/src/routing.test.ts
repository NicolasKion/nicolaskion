// The routing contract: frontend/proxy-paths.ts, deploy/Caddyfile.dev and
// deploy/Caddyfile name the same paths.
import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

import { axumPrefixes, updateFeedPrefix } from '../proxy-paths.ts';

const caddyfiles = ['Caddyfile', 'Caddyfile.dev'].map((name) => ({
  name,
  text: readFileSync(new URL(`../../deploy/${name}`, import.meta.url), 'utf8'),
}));

describe('the Caddyfiles', () => {
  it.each(caddyfiles)('$name sends the api paths to Axum', ({ text }) => {
    const matcher = text.match(/@axum path (.+)/)?.[1].split(' ');
    // `/up` is one page; the others are trees.
    expect(matcher).toEqual(
      axumPrefixes.map((prefix) => (prefix === '/up' ? prefix : `${prefix}/*`)),
    );
  });

  it.each(caddyfiles)('$name serves the update feed from disk as octet-stream', ({ text }) => {
    expect(text).toContain(`handle_path ${updateFeedPrefix}* {`);
    expect(text).toMatch(
      /@binary path \*\.gz \*\.sig\n\t\theader @binary Content-Type application\/octet-stream\n\t\tfile_server\n/,
    );
  });

  it('serves the feed from the host directory in production', () => {
    expect(caddyfiles[0].text).toContain('root * /srv/nicolaskion/update-feed');
    expect(caddyfiles[0].text).toContain(
      'www.nicolaskion.dev {\n\tredir https://nicolaskion.dev{uri} permanent\n}',
    );
  });
});
