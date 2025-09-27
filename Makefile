# Makefile for Paysys project
# Requires sqlx-cli installed: cargo install sqlx-cli

# Ledger Database URL
LEDGER_DB_URL=postgres://user:user123@localhost:5432/ledger

# Account Database URL
ACCOUNTS_DB_URL=postgres://user:user123@localhost:5433/accounts

# Docker Compose file
DC_FILE=build/docker-compose.yml

# Proto folder
PROTO_DIR=proto

.PHONY: help db create-db new-migration run-migrations up down generate-proto migrate

help:
	@echo "Usage:"
	@echo "  make up                 Start Docker services"
	@echo "  make down               Stop Docker services"
	@echo "  make create-db          Create databases if they do not exist"
	@echo "  make new-migration NAME=descriptive_name TARGET=accounts|ledger  Create a new migration"
	@echo "  make run-migrations     Apply all pending migrations"
	@echo "  make generate-proto     Generate Rust protobuf code"

# Start Docker services
up:
	docker-compose -f $(DC_FILE) up -d

# Stop Docker services
down:
	docker-compose -f $(DC_FILE) down

# Create both databases
db: create-db

# Create databases individually
create-db: create-accounts-db create-ledger-db

create-accounts-db:
	sqlx database create --database-url $(ACCOUNTS_DB_URL)

create-ledger-db:
	sqlx database create --database-url $(LEDGER_DB_URL)

# Create a new migration
new-migration:
ifndef NAME
	$(error NAME is not set. Usage: make new-migration NAME=descriptive_name TARGET=accounts|ledger)
endif
ifndef TARGET
	$(error TARGET is not set. Usage: make new-migration NAME=descriptive_name TARGET=accounts|ledger)
endif
	@echo "Creating new migration '$(NAME)' for $(TARGET)"
	sqlx migrate add $(NAME) --source migrations/$(TARGET)

# Run migrations individually
migrate-accounts:
	sqlx migrate run --database-url $(ACCOUNTS_DB_URL) --source migrations/accounts

migrate-ledger:
	sqlx migrate run --database-url $(LEDGER_DB_URL) --source migrations/ledger

# Run all migrations
run-migrations: migrate-accounts migrate-ledger

# Generate protobufs into ledger-proto/gen/
generate-proto:
	cd $(PROTO_DIR) && buf mod update && buf generate

lint:
	@echo "Linting all projects with cargo"
	@rustup component add clippy 2> /dev/null
	cargo clippy --package  accounts --all-targets --all-features -- -D warnings

nextest:
	@echo "Testing all projects with cargo nextest"
	cargo nextest run --release -p accounts --retries 2

watch:
	@echo "Starting cargo watch with cargo nextest"
	cargo watch -x check -x 'nextest run --release -p accounts --retries 2'

test:
	@echo "Testing all projects with cargo test"
	cargo test --release -p accounts