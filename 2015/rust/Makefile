BUILD = build
INPUTS = ../inputs

SRCS = $(wildcard *.rs)
BINS = $(basename $(SRCS))
RUNS = $(addsuffix .run, $(BINS))

.PHONY: default
default: all

$(BUILD)/day%.rs: %.rs | $(BUILD)
	cp $^ $@

# Build one .rs file project
# ex: `make 01`
.PHONY: $(BINS)
$(BINS): %: $(BUILD)/day%.rs
	cargo single build --release $^

# Build all .rs file projects
.PHONY: all
all: $(sort $(BINS))

# Run 1 binary
# ex: `make 01.run`
.PHONY: $(RUNS)
$(RUNS): %.run: $(BUILD)/day%.rs
	cargo single run $^ $(INPUTS)/$*.txt

# Run all binaries
.PHONY: all.run
all.run: | $(sort $(RUNS))

$(BUILD):
	mkdir $(BUILD)

# Clean
.PHONY: clean
clean:
	rm -rf $(BUILD)
