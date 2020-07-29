.PHONY: ci-js ci-rust build build-js build-rust deps deploy fmt update serve test-js test-rust lint-js lint-rust

# Is run on CI/CD
ci-js: deps lint-js test-js
ci-rust: lint-rust test-rust

# Build both JS and Rust code
build: build-js build-rust

# Build JS code
build-js:
	yarn run webpack --config ./www/webpack.config.js

# Build Rust code
build-rust:
	wasm-pack build

deploy:
	git push heroku master

# Install JS dependencies
deps:
	cd www && yarn install --skip-integrity-check --non-interactive --no-progress

# Fix all linting errors
fmt:
	rustfmt src/*.rs
	yarn run eslint --fix www/*.js

# Run when JS code should pick up Rust code changes
update: build-rust
	cd www && yarn upgrade wasm-impossible-tic-tac-toe

# Start webpack server
serve:
	cd www && yarn run webpack-dev-server --config ./www/webpack.config.dev.js

test-js:
	cd www; exit 0

test-rust:
	cargo test
	wasm-pack test --firefox --headless
	wasm-pack test --chrome  --headless

lint-js:
	yarn run eslint www/*.js

lint-rust:
	cargo fmt --all -- --check
