ENDPOINT ?= bitcoin.substreams.pinax.network:443
ROOT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# Include environment variables from .env.local if it exists
ifneq (,$(wildcard ./.env.local))
    include .env.local
    export
endif

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./proto/analytics.proto --exclude-paths="sf/substreams,google"

.PHONY: pack
pack: build
	substreams pack

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block_metrics -s 800000 -t +10

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_block_metrics -s 800000 -t +10 --limit-processed-blocks 0

.PHONY: info
info:
	substreams info substreams.yaml

.PHONY: codegen
codegen: protogen
	cargo build

.PHONY: test
test:
	cargo test -- --nocapture

.PHONY: clean
clean:
	cargo clean
	rm -rf ./src/pb/bitcoin
