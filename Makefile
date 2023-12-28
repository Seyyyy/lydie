debug-build:
	@echo "Building..." \
	&& wasm-pack build --target bundler --out-dir dist/pkg --debug

build:
	@echo "Building..." \
	&& wasm-pack build --target bundler --out-dir dist/pkg --release

pack:
	@echo "Packaging..." \
	&& wasm-pack build --target bundler --out-dir dist/pkg --debug \
	&& cp -f README.md dist/README.md \
	&& cp -f LICENSE dist/LICENSE \
	&& rm -f dist/pkg/LICENSE \
	&& rm -f dist/pkg/README.md \
	&& cd dist \
	&& npm pack

publish:
	@echo "Publishing..." \
	&& wasm-pack build --target bundler --out-dir dist/pkg --release \
	&& cp -f README.md dist/README.md \
	&& cp -f LICENSE dist/LICENSE \
	&& rm -f dist/pkg/LICENSE \
	&& rm -f dist/pkg/README.md \
	&& cd dist \
	&& npm publish

lint:
	@echo "Linting..." \
	&& cargo clippy --all-targets --all-features -- -A warnings

e2e-test:
	@echo "Testing..." \
	&& wasm-pack build --target bundler --out-dir dist/pkg --release \
	&& npm run test:e2e