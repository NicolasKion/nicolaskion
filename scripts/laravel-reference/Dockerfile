# The Laravel app from `main`, runnable locally as the reference the
# golden fixtures are recorded from (see run.sh). PHP is never installed
# on the host.
FROM php:8.4-cli
RUN apt-get update \
    && apt-get install -y --no-install-recommends unzip git libzip-dev libsqlite3-dev \
    && docker-php-ext-install zip pcntl bcmath \
    && rm -rf /var/lib/apt/lists/*
COPY --from=composer:2 /usr/bin/composer /usr/bin/composer
# Node builds the Vite assets, so the reference also renders in a browser
# (heads and screenshots).
COPY --from=node:26-bookworm /usr/local/bin/node /usr/local/bin/node
COPY --from=node:26-bookworm /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm \
    && ln -s /usr/local/lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx
WORKDIR /app
