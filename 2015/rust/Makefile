BUILD = build
INPUTS = ../inputs

SRCS = $(wildcard *.rs)
BINS = $(basename $(SRCS))
RUNS = $(addsuffix .run, $(BINS))

.PHONY: default
default: all

# Build one .hs file
# ex: `make 01`
.PHONY: $(BINS)
$(BINS): %: %.rs | $(BUILD)
	cp $^ $(BUILD)/day$^
	cargo single build $(BUILD)/day$^

# Build all .hs files
.PHONY: all
all: $(sort $(BINS))

# Run 1 .hs file
# ex: `make 01.run`
.PHONY: $(RUNS)
$(RUNS): %.run: %
	cargo single run $(BUILD)/day$^ $(INPUTS)/day$*.txt

.PHONY: all.run
all.run: | $(sort $(RUNS))

$(BUILD):
	mkdir $(BUILD)

# Clean
.PHONY: clean
clean:
	rm -rf $(BUILD)
