build: ## build
	cargo build

test: ## test
	cargo test

run: ## run
	RUST_BACKTRACE=1 cargo run -- -i tests/add.forth

clean: ## clean all the things
	echo implement clean makefile rule

work: ## open all files in editor
	emacs  Makefile src/* 

debug:
	gdb --args ./target/debug/ruforth -i tests/add.forth

# http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk \
	'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

FORCE:

