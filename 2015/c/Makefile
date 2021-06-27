BUILD = build
INPUTS = ../inputs

SRCS = $(wildcard *.c)
BINS = $(basename $(SRCS))
RUNS = $(addsuffix .run, $(BINS))

.PHONY: default
default: all

$(BUILD)/%: %.c | $(BUILD)
	gcc -o $@ $^

# Build one .c file
# ex: `make 01`
.PHONY: $(BINS)
$(BINS): %: $(BUILD)/%

# Build all .c files
.PHONY: all
all: $(sort $(BINS))

# Run 1 binary
# ex: `make 01.run`
.PHONY: $(RUNS)
$(RUNS): %.run: $(BUILD)/%
	$^ $(INPUTS)/$*.txt

# Run all binaries
.PHONY: all.run
all.run: | $(sort $(RUNS))

$(BUILD):
	mkdir $(BUILD)

# Clean
.PHONY: clean
clean:
	rm -rf $(BUILD)
