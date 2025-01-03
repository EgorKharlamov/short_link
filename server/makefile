db_up:
	docker compose -f db.docker-compose.yml up -d --remove-orphans
db_down:
	docker compose -f db.docker-compose.yml down

run: db_up
	cargo run --bin server
