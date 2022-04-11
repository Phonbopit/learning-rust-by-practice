build:
	cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program
deploy:
	solana program deploy dist/program/mini_solana.so