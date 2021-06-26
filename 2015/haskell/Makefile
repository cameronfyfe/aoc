SRC = src
INPUTS = ../inputs

SRCS = $(wildcard $(SRC)/*.hs)
BINS = $(patsubst $(SRC)/%.hs,%,$(SRCS))
RUNS = $(patsubst %,%.run,$(BINS))

.PHONY: default
default: all

# Build one .hs file
# ex: `make 01`
%:: $(SRC)/%.hs
	ghc -o $@ $^

# Build all .hs files
.PHONY: all
all: $(BINS)

# Run 1 .hs file
# ex: `make 01.run`
# recursive make because of issue with pattern rules/phony combo
.PHONY: $(RUNS)
$(RUNS):
	@$(MAKE) --no-print-directory $(basename $@)
	./$(basename $@) $(INPUTS)/day$(basename $@).txt

# Clean
.PHONY: clean
clean:
	rm -f $(BINS)
	rm -f $(SRC)/*.o
	rm -f $(SRC)/*.hi
