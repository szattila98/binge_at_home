volta

# General dev setup

## Prerequisites

- [_just_](https://github.com/casey/just) 1.14.0 or higher
  - just is a project specific command runner. It runs the commands defined in the justfile at the project root. It helps development by setting up the environment among other things.
  - in the project root run `just` to list commands

## Recommended configuration

- Run `just install-cargo-tooling` to install the recommended cargo tools for development. Requires _cargo_.
- Run `just add-git-hook` to install the recommended git hook locally. Requires _cocogitto_ (it is installed in the previous step).

# Backend dev setup

## Prerequisites

- _docker_ 24.0.2 or higher
- _docker-compose_ 2.18.1 or higher
- _cargo_ 1.70.0 or higher

### IDE

- _Visual Studio Code_ is recommended.
  - with rust-analyzer (rust-lang.rust-analyzer) extension to work with rust
  - with CodeLLDB (vadimcn.vscode-lldb) extension to debug

### Recommended cargo tooling

- _cargo-watch_ - hot-reload for development, use with `cargo watch -x run`
- _cargo-audit_ - vulnerability scanner, use with `cargo audit`
- _cargo-llvm-cov_ - test coverage tool, use with `cargo llvm-cov --html`
- _cargo-edit_ - `cargo upgrade --workspace --to-lockfile` automatically updates dependencies in Cargo.toml
- _sqlx-cli_ - sqlx helper cli
- _cocogitto_ - conventional commit toolbox

## Quick start with `just docker-*` commands

To quickly start using docker compose, use these justfile commands.
These will also rebuild containers on file changes, if ran again.

- `docker-up-all` - it will start all the services.
- `docker-up-server` - it will run the server and its dependencies, usable for local frontend development.
- `docker-up-dev` - it will run the server dependencies. Usable for local backend development.
- `down` - it will stop every service

## Running server

- Build and run with `cargo run`, it will automatically run database migrations
  - When developing use `cargo watch -x run` for hot-reloading, provided `cargo-watch` is installed
  - Do not forget that you will need a database up and running to properly run the application. The easiest way is to run `just docker-up-dev` in the root of the project
- Migrations will run automatically on application startup.

## Use the API

- There is a Postman collection file, ready to be used for testing during development at `postman_collections.json`
  - Don't just use it, remember to update it when any of the API schema changes.

# Frontend dev setup

## Prerequisites

- _docker_ 24.0.2 or higher
- _docker-compose_ 2.18.1 or higher
- _node_ 20.3.0 or higher
- _npm_ 9.7.1 or higher

### Recommended tool

- [_volta_](https://volta.sh/) - a node version manager which automatically determines used tooling versions based on the `package.json` volta entry.

## Starting backend for client

- To run the complete backend (database & server), use `just docker-up-server`, the database will be available at port 3306 and the server on 8000
- The build will take some time at first and on changes, consequent runs will be much faster thanks to docker caching
- It can be used as a backend for frontend development, the server is built in release mode, so it is smaller and highly optimized

## Starting Svelte dev server

- `cd ./client`
- `npm i`
- `npm run dev`
