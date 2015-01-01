
.PHONY: test

test: ./target/ion
	python ./test/test.py --pos ./test/iter1/good/ ./target/ion
	python ./test/test.py --neg ./test/iter1/bad/ ./target/ion

./target/ion: ./src/*.rs
	cargo build
