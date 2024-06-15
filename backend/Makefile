.PHONY: setup up install_binstall install_watch install_nextest install_sqlx test_oneshot test run fmt clippy clippy_check stack_up wait_stack stack_down stop destroy sqlx_prepare migrate migrate_add psql

ifneq (,$(wildcard ./.local.env))
    include .local.env
    export
endif


use_env_test:
ifneq (,$(wildcard ./.tests.env))
    include .tests.env
    export
endif

setup:
	@cargo --version >/dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && source "${HOME}/.cargo/env"
	@rustup default 1.75 >/dev/null
	@$(MAKE) install_binstall
	@$(MAKE) install_watch
	@$(MAKE) install_nextest
	@$(MAKE) install_sqlx
	@echo "\nSetup completed! Remember to add the following line to your ~/.zshrc or ~/.bashrc:\n    source \"$\{HOME}/.cargo/env\""

install_binstall:
	@cargo binstall --help >/dev/null || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

install_watch:
	@cargo binstall -y -q cargo-watch

install_nextest:
	@cargo binstall -y -q cargo-nextest

install_sqlx:
	cargo sqlx --version | grep "${SQLX_VERSION}" >/dev/null || cargo install sqlx-cli --no-default-features --features rustls,postgres --version "${SQLX_VERSION}"

test_ci:
	cargo nextest run --test-threads 1

test_oneshot:
	cargo nextest run

test: test_oneshot

run: stack_up migrate
	cargo watch -q -c -x 'run --bin api'

local/run: stack_up migrate_local
	cargo watch -q -c -x 'run --bin api'

fmt:
	cargo fmt --all

clippy:
	cargo clippy --fix --all-targets --all-features --allow-staged --allow-dirty -- -Dwarnings -Dclippy::unwrap_used

clippy_check:
	cargo clippy --all-targets --all-features -- -Dwarnings -Dclippy::unwrap_used

stack_up:
	@docker-compose up -d
	@$(MAKE) wait_stack

wait_stack:
	@while [ $$(docker-compose ps | grep postgres | grep Up | wc -l) -eq 0 ]; do echo "waiting for stack to be healthy..."; sleep 1; done

up: stack_up migrate

stack_down:
	docker-compose down

stop: stack_down

destroy: stack_down
	-docker volume rm unico-service_pgdata

sqlx_prepare:
	cargo sqlx prepare

migrate:
	cargo sqlx migrate run --source database/migrations --database-url ${DATABASE_URL}
	echo "processed all migrations"

migrate_local:
	cargo sqlx migrate run --source database/migrations --database-url postgres://local:local@localhost:5432/login
	echo "processed all migrations local"

migrate_add:
	cargo sqlx migrate add ${NAME} --source database/migrations

psql:
	psql $(DATABASE_URL)
