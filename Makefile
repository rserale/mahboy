PROJECT_NAME := mahboy

build:
	cargo build

run: ## Exécute le projet en mode debug
	cargo run

clean:
	cargo clean