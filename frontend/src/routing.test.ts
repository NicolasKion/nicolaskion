// The routing contract: frontend/proxy-paths.ts and deploy/Caddyfile name
// the same paths.
import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

import { axumPrefixes, updateFeedPrefix } from '../proxy-paths.ts';

const caddyfile = readFileSync(new URL('../../deploy/Caddyfile', import.meta.url), 'utf8');

describe('the Caddyfile', () => {
  it('sends the api paths to Axum', () => {
    const matcher = caddyfile.match(/@axum path (.+)/)?.[1].split(' ');
    // `/up` is one page; the others are trees.
    expect(matcher).toEqual(
      axumPrefixes.map((prefix) => (prefix === '/up' ? prefix : `${prefix}/*`)),
    );
  });

  it('serves the update feed from the host directory as octet-stream', () => {
    expect(caddyfile).toContain(`handle_path ${updateFeedPrefix}* {`);
    expect(caddyfile).toContain('root * /srv/nicolaskion/update-feed');
    expect(caddyfile).toMatch(
      /@binary path \*\.gz \*\.sig\n\t\theader @binary Content-Type application\/octet-stream\n\t\tfile_server\n/,
    );
  });

  it('redirects www to the apex', () => {
    expect(caddyfile).toContain(
      '@www header_regexp www Host ^www\\.(.+)$\n\tredir @www https://{re.www.1}{uri} permanent\n',
    );
  });
});
