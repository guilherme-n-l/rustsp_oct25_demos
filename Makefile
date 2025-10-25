define generate-md-from-scripts
	@echo Generating Docs for $(1); \
	for f in $(wildcard $(1)/*.$(2)); do \
		echo -e "## $$f\n" >> $(1)/README.md; \
		echo '```$(2)' >> $(1)/README.md; \
		cat $$f >> $(1)/README.md; \
		echo '```' >> $(1)/README.md; \
		echo "" >> $(1)/README.md; \
	done
endef

all: nix ffi bindgen docs

nix:
	$(MAKE) -C $@

ffi:
	$(MAKE) -C $@

bindgen:
	$(MAKE) -C $@

demo:
	$(MAKE) KDIR=$$PWD/demo/linux -C $@

docs:
	@for dir in *; do \
		if [ -d $$dir ]; then \
			$(MAKE) docs-$$dir; \
		fi \
	done
docs-%:
	@-$(MAKE) -C $(*) clean
	$(MAKE) clean-docs-$(*)
	$(MAKE) generate-docs-$(*)

generate-docs-nix:
	$(call generate-md-from-scripts,nix,sh)
generate-docs-ffi:
	$(call generate-md-from-scripts,ffi,rs)
	$(call generate-md-from-scripts,ffi,c)
generate-docs-bindgen:
	$(call generate-md-from-scripts,bindgen,h)
	$(call generate-md-from-scripts,bindgen,c)
	$(call generate-md-from-scripts,bindgen,rs)
generate-docs-demo:
	$(call generate-md-from-scripts,demo,sh)
	$(call generate-md-from-scripts,demo,rs)

clean-docs:
	@for dir in *; do \
		if [ -d $$dir ]; then \
			$(MAKE) clean-doc-$$dir; \
		fi \
	done
clean-docs-%:
	@echo Cleaning Docs for $(*); \
		cd $(*) && \
		echo -n "" > README.md

clean:
	@while IFS= read -r pattern; do \
		git ls-files --ignored --exclude-standard --others -- "$$pattern" | xargs rm -rf; \
	done < .gitignore
	-@for dir in */; do \
		if [ -f $$dir/Makefile ]; then \
			echo "Cleaning in $$dir"; \
			$(MAKE) -C $$dir clean; \
		fi \
	done


.PHONY: all nix clean docs-% docs clean-docs-% clean-docs generate-docs-%
