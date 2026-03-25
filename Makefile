build:
	cargo build
	g++ -std=c++11 check.cpp -o check

clean:
	cargo clean
	rm -f PRINTER* LOG check

runHW8: build
	cargo run -- -1 -1 -1

testHW8: runHW8
	./check 1 PRINTER0

runHWSmall: build
	cargo run -- -4 -2 -3

testHWSmall: runHWSmall
	./check 4 PRINTER0 PRINTER1 PRINTER2

runHW9: build
	cargo run -- -26 -2 -3

testHW9: runHW9
	./check 26 PRINTER0 PRINTER1 PRINTER2