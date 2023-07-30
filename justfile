_default:
  @just --list --unsorted

_check-app app:
    @if ! [ -x "$(command -v {{app}})" ]; then \
        echo "\033[1;31m{{app}} is not installed ✘!\033[0m"; \
        exit 1; \
    fi

install-cargo-tooling:
    # Installing (or if installed updating) cargo tooling
    @just _check-app cargo
    @cargo install cargo-watch cargo-audit cargo-llvm-cov cargo-edit sqlx-cli cocogitto 

add-git-hook:
    # Adding Cocogitto hook to local repository
    @just _check-app cog
    @cog install-hook pre-push

docker-up-all:
    # Starting client and dependencies in docker-compose
    @just _check-app docker-compose
    @docker-compose up -d --build

docker-up-server:
    # Starting server and dependencies in docker-compose
    @just _check-app docker-compose
    @docker-compose up server -d --build

docker-up-dev:
    # Starting development services (database)
    @just _check-app docker-compose
    @docker-compose up database -d --build

docker-down:
    # Stopping docker-compose environment
    @just _check-app docker-compose
    @docker-compose down

lint-server-seriously:
    # Scanning server files with pedantic and nursery clippy rules
    @cd server && cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -A clippy::missing_errors_doc -A clippy::missing_const_for_fn -A clippy::must_use_candidate
