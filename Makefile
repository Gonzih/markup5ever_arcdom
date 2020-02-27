BACKTRACE ?= 0
CARGO = cargo --color always
CARGO_ARGS = $(if $(RELEASE),--release) $(if $(STATIC_BINARY), --target=x86_64-unknown-linux-musl)

.PHONY: test-nix
test-nix:
	nix-shell shell.nix --run 'make test'

.PHONY: build-nix
build-nix:
	nix-shell shell.nix --run 'make build'

.PHONY: build
build:
	$(CARGO) build $(CARGO_ARGS)

.PHONY: test
test:
	$(CARGO) test $(CARGO_ARGS)

.PHONY: shell
shell:
	nix-shell shell.nix

rust-setup:
	rustup default nightly

publish:
	nix-shell shell.nix --run 'cargo publish'

submodules:
	git submodule init
	git submodule update

AUTH_HEADER="$(shell git config --local --get http.https://github.com/.extraheader)"
setup-git-for-github:
	git submodule sync --recursive
	git -c "http.extraheader=$(AUTH_HEADER)" -c protocol.version=2 submodule update --init --force --recursive --depth=1

test-via-docker:
	docker run -t -v $(shell pwd):/workdir nixos/nix sh -c 'cd /workdir && nix-shell shell.nix --run "make rust-setup test"'
