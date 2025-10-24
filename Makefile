define generate-md-from-scripts
	@echo "# $(1) Scripts" > $(1)/README.md; \
	for f in $(wildcard $(1)/*.$(2)); do \
		echo >> $(1)/README.md; \
		echo "## $$f" >> $(1)/README.md; \
		echo '```$(2)' >> $(1)/README.md; \
		cat $$f >> $(1)/README.md; \
		echo '```' >> $(1)/README.md; \
	done
endef

nix:
	@cd nix && for f in *.sh; do \
		echo "Running $$f"; \
		bash $$f; \
	done


docs:
	@for dir in *; do \
		if [ -d $$dir ]; then \
			$(MAKE) docs-$$dir; \
		fi \
	done

docs-%:
	$(call generate-md-from-scripts,$(*),sh)

clean:
	@while IFS= read -r file; do \
		rm -rf $$file; \
	done < .gitignore


.PHONY: nix clean docs-% docs
