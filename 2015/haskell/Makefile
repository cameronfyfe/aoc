INPUTS = ../inputs

SRCS = $(wildcard *.hs)
BINS = $(basename $(SRCS))
RUNS = $(addsuffix .run, $(BINS))

.PHONY: default
default: all

# Build one .hs file
# ex: `make 01`
$(BINS): %: %.hs
	ghc -o $@ $^

# Build all .hs files
.PHONY: all
all: $(BINS)

# Run 1 .hs file
# ex: `make 01.run`
.PHONY: $(RUNS)
$(RUNS): %.run: %
	./$* $(INPUTS)/day$*.txt

# Clean
.PHONY: clean
clean:
	rm -f $(BINS)
	rm -f *.o
	rm -f *.hi
