.ONE_SHELL:
.SHELL = /bin/bash


# Exporting some additional env variables, so some of the dependicies could be built for wasm32 target.
ifeq ($(shell uname),Darwin)
export CC_wasm32_unknown_unknown := $(shell brew --prefix llvm)/bin/clang
export AR_wasm32_unknown_unknown := $(shell brew --prefix llvm)/bin/llvm-ar
endif


.PHONY: build-wasm
build-wasm: # Build the wasm project.
	@cd viewer-wasm/ && wasm-pack build --release --target web
	@mkdir -p frontend/static/wasm
	@cp viewer-wasm/pkg/viewer_wasm* frontend/static/wasm/

.PHONY: serve
serve: # Start the dev web server and serve the root folder of the project.
	@cd frontend/ && npm run dev

.PHONY: run
run: build-wasm serve # Build the module and start the dev server.

.PHONY: format-cargo
format-cargo:
	@cd viewer-wasm && cargo fmt

.PHONY: format-npm
format-npm:
	@cd frontend && npm run format

.PHONY: format
format: format-cargo format-npm # Format the proejct's files.

.PHONY: lint
lint: # Run linting checks on frontend code.
	@cd frontend && npm run lint

.PHONY: install-npm
install-npm: # Install the npm dependicies.
	@cd frontend && npm ci

.PHONY: release
release: install-npm format lint build-wasm release-clean # Build production release for GitHub Pages deployment.
	@echo "Starting production release build..."
	@echo "Step 3: Preparing WASM files for static deployment..."
	@mkdir -p frontend/static/wasm
	@cp viewer-wasm/pkg/viewer_wasm* frontend/static/wasm/
	@echo "Step 4: Building frontend for production..."
	@cd frontend && NODE_ENV=production npm run build
	@echo "Step 5: Creating release directory..."
	@mkdir -p release
	@cp -r frontend/build/* release/
	@echo "Step 6: Verifying release build..."
	@ls -la release/
	@echo "Production release build complete! Files ready for deployment in ./release/"


.PHONY: release-clean
release-clean: # Remove the release directory
	@rm -rf ./release/


.PHONY: release-run
release-run: release # Create a release and start http server to run it
	@npx --yes http-server -c-1 release
