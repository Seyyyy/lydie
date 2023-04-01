pack:
	@echo "Packaging..." \
	&& wasm-pack build --target web --out-dir dist/pkg --release \
	&& cp -f README.md dist/README.md \
	&& cp -f LICENSE dist/LICENSE \
	&& rm -f dist/pkg/LICENSE \
	&& rm -f dist/pkg/README.md \
	&& cd dist \
	&& npm pack
