.DEFAULT_GOAL := show-help
THIS_FILE := $(lastword $(MAKEFILE_LIST))
ROOT_DIR:=$(shell dirname $(realpath $(THIS_FILE)))

.PHONY: show-help
# See <https://gist.github.com/klmr/575726c7e05d8780505a> for explanation.
## This help screen
show-help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)";echo;sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f|awk -F --- -v n=$$(tput cols) -v i=29 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'

.PHONY: test
## Test it was built ok
test:
	UST_BACKTRACE=1 cargo test

.PHONY: bench
## Benchmark
bench:
	cargo bench

.PHONY: smoke-test
## Run a smoke test and see if the app runs
smoke-test: build
	cargo run --bin git-mit -- -h
	cargo run --bin git-mit-config -- -h
	cargo run --bin git-mit-relates-to -- -h
	cargo run --bin git-mit-install -- -h
	cargo run --bin mit-commit-msg -- -h
	cargo run --bin mit-pre-commit -- -h
	cargo run --bin mit-prepare-commit-msg -- -h

.PHONY: build
## Build release version
build:
	cargo build --release

.PHONY: lint
## Lint it
lint:
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo -A clippy::multiple-crate-versions
	cargo check
	cargo audit
	npx prettier --check **.yml

.PHONY: fmt
## Format what can be formatted
fmt:
	cargo +nightly  fix --allow-dirty
	cargo +nightly clippy --allow-dirty --fix -Z unstable-options --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::nursery
	cargo +nightly fmt --all
	npx prettier --write **.yml

.PHONY: clean
## Clean the build directory
clean:
	cargo clean
