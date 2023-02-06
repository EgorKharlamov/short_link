db_up:
	docker-compose -f db.docker-compose.yml up -d
db_down:
	docker-compose -f db.docker-compose.yml down

run: db_up
	cargo run --bin server
