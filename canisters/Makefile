setup:
	cargo install cargo-audit
	cargo install candid-extractor

gen:
	dfx generate escrow

build:
	cargo build
	candid-extractor target/wasm32-unknown-unknown/debug/escrow.wasm > src/escrow/escrow.did
	make gen
	
deploy:
	make build
	dfx deploy

.PHONY: deploy prev
deploy-play:
	make build
	dfx deploy --playground

deploy-prod:
	make build
	dfx deploy --network=ic
