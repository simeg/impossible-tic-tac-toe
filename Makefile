.PHONY: ci build build-js build-rust deps fmt update serve test lint

# Is run on CI/CD
ci: deps lint test

# Build both JS and Rust code
build: build-js build-rust

# Build JS code
build-js:
	cd www && yarn run webpack --config webpack.config.js

# Build Rust code
build-rust:
	wasm-pack build

# Install JS dependencies
deps:
	cd www; yarn install --skip-integrity-check --non-interactive --no-progress

# Fix all linting errors
fmt:
	rustfmt src/*.rs
	cd www; yarn run eslint --fix *.js

# Run when JS code should pick up Rust code changes
update: build
	cd www && yarn upgrade wasm-impossible-tic-tac-toe

# Start webpack server
serve:
	cd www && yarn run webpack-dev-server

# Run tests
test:
	cargo test
	# cd www; jest

# Run linters
lint:
	cargo fmt --all -- --check
	cd www; yarn run eslint *.js
