
all:
	cargo build --quiet

dev:
	cargo run ./scop/src/resources/42/42.obj --quiet

test:
	cargo test

