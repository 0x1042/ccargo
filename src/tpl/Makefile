PRESET ?= asan

RED=\033[31m
GREEN=\033[32m
YELLOW=\033[33m
RESET=\033[0m

START := $(shell date +%s)

.PHONY: all config build clean info

all: build
	@END=$$(date +%s); \
	ELAPSED=$$((END - $(START))); \
	printf "$(GREEN)%s  build time cost: %s seconds$(RESET)\n" "$$(date '+%Y-%m-%d %H:%M:%S')" "$${ELAPSED}"

info:
	cmake --list-presets
	@printf "\n"

config: info
	@printf "$(YELLOW)%s Configuring with preset: %s$(RESET)\n" "$$(date '+%Y-%m-%d %H:%M:%S')" "$(PRESET)"
	mkdir -p build/$(PRESET) && cmake --preset=$(PRESET)

build: config
	@printf "$(GREEN)%s Building with preset: %s$(RESET)\n" "$$(date '+%Y-%m-%d %H:%M:%S')" "$(PRESET)"
	cmake --build build/$(PRESET)
	ln -sf build/$(PRESET)/compile_commands.json .

clean:
	@printf "$(RED)%s Cleaning build for: %s$(RESET)\n" "$$(date '+%Y-%m-%d %H:%M:%S')" "$(PRESET)"
	rm -rf build/$(PRESET)
	rm -f compile_commands.json
