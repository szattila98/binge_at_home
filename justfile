_default:
  @just --list --unsorted

_check-app app:
    @if [ -x "$(command -v {{app}})" ]; then \
        echo "\033[1;32m{{app}} ✓\033[0m"; \
    else \
        echo "\033[1;31m{{app}} ✘\033[0m"; \
    fi

_check-cargo app:
    @if [[ "$(cargo install --list)" == *"{{app}}"* ]]; then \
        echo "\033[1;32m{{app}} ✓\033[0m"; \
    else \
        echo "\033[1;31m{{app}} ✘\033[0m"; \
    fi

check-devtools:
    # Checking installment of tools needed for development
    @just _check-app node
    @just _check-app cargo
    @just _check-app docker-compose
    # Checking installment of recommended tools
    @just _check-app yarn
    @just _check-app volta
    # Checking recommended cargo tooling
    @just _check-cargo cargo-watch
    @just _check-cargo cargo-audit
    @just _check-cargo cargo-llvm-cov
    @just _check-cargo cargo-edit
    @just _check-cargo sqlx-cli
    @just _check-cargo cocogitto
    # Refer to DEV.md for version information

install-cargo-tooling:
    # Installing (or if installed updating) cargo tooling
    @cargo install cargo-watch cargo-audit cargo-llvm-cov cargo-edit sqlx-cli cocogitto

setup-cocogitto-hook:
    # Cocogitto hook for pre-push
    cog install-hook pre-push

docker-start-all:
    # Starting client and dependencies in docker-compose
    docker-compose up -d --build

docker-start-server:
    # Starting server and dependencies in docker-compose
    docker-compose up server -d --build

docker-start-dev:
    # Starting development services (database)
    docker-compose up database -d --build

docker-stop:
    # Stopping docker-compose environment
    docker-compose down