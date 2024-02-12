setup:
	cargo install cargo-audit
	cargo install candid-extractor

gen:
	dfx generate
	cp -rf src/declarations ../web_lab/src/lib/canisters/.

build:
	cargo build
	candid-extractor target/wasm32-unknown-unknown/debug/escrow.wasm > src/escrow/escrow.did	
	
deploy:
	make build
	dfx deploy
	make gen

.PHONY: deploy prev
deploy-play:
	make build
	dfx deploy --playground

deploy-prod:
	make build
	dfx deploy --network=ic
