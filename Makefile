.PHONY: subdate

subdate:
	git submodule update --init --recursive
	git submodule update --remote