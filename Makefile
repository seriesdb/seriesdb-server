.PHONY : run compile gen test clean

CARGO=cargo

run: gen compile
	RUST_BACKTRACE=1 ${CARGO} run -- --nocapture

compile:
	${CARGO} build --color=always --all --all-targets

release: gen
	${CARGO} build --release --color=always --all --all-targets && bin/release.sh

gen:
	protocol-generator/bin/gen_protocol_code.sh

test:
	RUST_BACKTRACE=1 ${CARGO} test -- --nocapture

clean:
	protocol-generator/bin/clean_protocol_code.sh
	${CARGO} clean --target-dir=protocol-generator/target
	${CARGO} clean
