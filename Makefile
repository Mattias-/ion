
.PHONY: test

test: ./target/ion
	python ./test/test.py --pos ./test/iter1/good/ ./target/debug/ion
	python ./test/test.py --neg ./test/iter1/bad/ ./target/debug/ion

./target/ion: ./src/*.rs
	cargo build
