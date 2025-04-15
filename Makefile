
all:
	cargo build --release

run:
	./target/release/scop ./scop/src/resources/42/42.obj 

dev:
	cargo run ./scop/src/resources/42/42.obj 

test:
	cargo test

