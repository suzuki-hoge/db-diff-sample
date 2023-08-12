run:
	@cargo run

build:
	@cargo clean
	@cargo build --release

up:
	@docker compose up --detach

down:
	@docker compose down --volumes

fix:
	@cargo +nightly fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged
	@make test

test:
	@cargo test
