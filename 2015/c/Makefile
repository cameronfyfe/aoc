build:
	mkdir build

.PHONY: build-1
build-1: | build
	gcc day01.c -o build/day01

.PHONY: run-1
run-1: build-1
	./build/day01 ../inputs/day01.txt

.PHONY: clean
clean:
	rm -rf build