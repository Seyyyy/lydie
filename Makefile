debug-build:
	@echo "Building..." \
	&& wasm-pack build --target bundler --out-dir dist/node --debug \
	&& wasm-pack build --target web --out-dir dist/web --debug

build:
	@echo "Building..." \
	&& wasm-pack build --target bundler --out-dir dist/node --release \
	&& wasm-pack build --target web --out-dir dist/web --release

pack:
	@echo "Packaging..." \
	&& cp -f README.md dist/README.md \
	&& cp -f LICENSE dist/LICENSE \
	&& wasm-pack build --target bundler --out-dir dist/node --debug \
	&& rm -f dist/node/LICENSE \
	&& rm -f dist/node/README.md \
	&& wasm-pack build --target web --out-dir dist/web --debug \
	&& rm -f dist/web/LICENSE \
	&& rm -f dist/web/README.md \
	&& cd dist \
	&& npm pack

publish:
	@echo "Publishing..." \
	&& cp -f README.md dist/README.md \
	&& cp -f LICENSE dist/LICENSE \
	&& wasm-pack build --target bundler --out-dir dist/node --release \
	&& rm -f dist/node/LICENSE \
	&& rm -f dist/node/README.md \
	&& wasm-pack build --target web --out-dir dist/web --debug \
	&& rm -f dist/web/LICENSE \
	&& rm -f dist/web/README.md \
	&& cd dist \
	&& npm publish

lint:
	@echo "Linting..." \
	&& cargo clippy --all-targets --all-features -- -A warnings

e2e-test:
	@echo "Testing..." \
	&& make build \
	&& npm run test:e2e