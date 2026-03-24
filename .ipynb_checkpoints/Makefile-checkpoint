build:
	cargo build
	g++ -std=c++11 check.cpp -o check

clean:
	cargo clean
	rm -f PRINTER* LOG check

testHW8: build
	cargo run -- -1 -1 -1
	./check 1 PRINTER0

testHWSmall: build
	cargo run -- -4 -2 -3
	./check 4 PRINTER0 PRINTER1 PRINTER2

testHW9: build
	cargo run -- -26 -2 -3
	./check 26 PRINTER0 PRINTER1 PRINTER2