ROOT_DIR := $(CURDIR)
export ROOT_DIR

.PHONY: codegen
codegen:
	$(MAKE) -C codegen install
	$(MAKE) -C codegen generate