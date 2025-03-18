BUILD_FOLDER ?= build
CMAKE ?= cmake

.PHONY: all build setup clean re

all: setup build

build:
	@$(CMAKE) --build $(BUILD_FOLDER)

setup:
	@$(CMAKE) -B $(BUILD_FOLDER)

clean:
	@rm -rf $(BUILD_FOLDER)

re: clean all

run: build
	@$(BUILD_FOLDER)/Harbour

install: build
	@$(CMAKE) --install $(BUILD_FOLDER)

help:
	@echo "Available targets:"
	@echo "  all      - Setup and build the project"
	@echo "  build    - Build the project"
	@echo "  setup    - Configure the build directory"
	@echo "  clean    - Remove build artifacts"
	@echo "  re       - Clean and rebuild the project"
	@echo "  run      - Build and run the project (update 'harbour')"
	@echo "  install  - Install the project (after build)"
	@echo "  help     - Show this help message"
