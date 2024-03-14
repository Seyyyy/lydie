pack:
	@echo "Packaging..." \
	&& npm run build \
	&& cd dist \
	&& npm pack

publish:
	@echo "Publishing..." \
	&& npm run build \
	&& cd dist \
	&& npm publish
