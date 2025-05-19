export APP_NAME := $(shell grep '^name =' Cargo.toml | awk -F'"' '{print $$2}')
PID := $(shell ps -ef | grep $(APP_NAME) | grep -v grep | awk "{print $$2}")
APP_CONTAINER_NAME := hk-smart-home-api
DB_CONTAINER_NAME := hk-smart-home-pg
REDIS_CONTAINER_NAME := hk-smart-home-redis

install:
	cargo add axum
	cargo add tokio -F full
	cargo add tower-http
	cargo add serde_json
	cargo add serde -F derive
	cargo add chrono -F serde
	cargo add dotenv
	cargo add uuid -F "serde v7"
	cargo add sqlx --features "runtime-async-std-native-tls mysql chrono uuid"
	cargo install cargo-watch
	cargo add validator --features derive
	cargo add axum-valid
	cargo add bcrypt
	cargo add jsonwebtoken
	cargo add axum-extra
	cargo add num_cpus
	cargo add sea-orm --features "sqlx-postgres runtime-tokio-rustls macros"
	cargo install sea-orm-cli
	cargo add strum --features "derive"
	cargo add bb8-redis
	cargo add bb8

stop-app:
	@if [ -n "$(PID)" ]; then \
		echo "Killing process $(PID)"; \
		kill -9 $(PID); \
	else \
		echo "No process found for $(APP_NAME)"; \
	fi


# ============================
#         MIGRATION
# ============================
migrate-generate-entity:
	@echo "Generating entities..."
	@sea-orm-cli generate entity -o entity/src

migrate-reset:
	@echo "Resetting database..."
	@sea-orm-cli migrate reset
	@make migrate-generate-entity

migrate-refresh:
	@echo "Refresh database..."
	@sea-orm-cli migrate refresh
	@make migrate-generate-entity

migrate-up:
	@echo "Migration..."
	@sea-orm-cli migrate up
	@make migrate-generate-entity

migrate-rollback:
	@echo "Rollback migration..."
	@sea-orm-cli migrate down


# ============================
#           SEEDERS
# ============================
seed-run:
	@echo "Start seeders..."
	@cargo run --bin seed


# ============================
#           CARGO
# ============================
cargo-run-prod:
	cargo build --release
	./target/release/${APP_NAME}

cargo-run-prod-restart: 
	make stop-app
	cargo-run-prod

cargo-run-dev:
	cargo run

cargo-run-dev-watch:
	cargo watch -q -c -w src/ -x run

cargo-clean:
	cargo clean

# ============================
#           DOCKER
# ============================
docker-cleanup:
	@echo "Cleaning up container..."
	@docker-compose down

docker-restart-dev:
	@echo "Restarting..."
	@make docker-cleanup
	@make install
	@make docker-run-dev

docker-restart-prod:
	@echo "Restarting..."
	@make docker-cleanup
	@make install
	@make docker-run-prod

docker-tail-logs:
	@echo "Tailing logs..."
	@docker logs -f ${APP_CONTAINER_NAME}

docker-jump-to-app:
	@echo "Jumping to app..."
	@docker exec -it $(APP_CONTAINER_NAME) sh

docker-jump-to-db:
	@echo "Jumping to db..."
	@docker exec -it $(DB_CONTAINER_NAME) mongosh

docker-jump-to-redis:
	@echo "Jumping to redis..."
	@docker exec -it $(REDIS_CONTAINER_NAME) sh

docker-run-dev:
	@echo "Starting dev env..."
	@-rm -f ~/.docker/config.json
	@sudo TARGET=dev docker-compose up -d --build || (echo "Error starting containers" && exit 1)
	@echo "Tailing logs..."
	@make docker-tail-logs

docker-run-prod:
	@echo "Starting prod env..."
	@-rm -f ~/.docker/config.json
	@ sudo TARGET=prod docker-compose up -d --build || (echo "Error starting containers" && exit 1)
	@echo "Tailing logs..."
	@make docker-tail-logs