SHELL := /bin/bash

# Variables
APP_NAME = my_app
SRC_DIR = src
BUILD_DIR = build

# Default target
all: build

# Example build step
build:
	@echo "🔧 Building..."
	@mkdir -p $(BUILD_DIR)
	@cp -r $(SRC_DIR)/* $(BUILD_DIR)/
	@echo "✅ Build completed."

# Run the app (example)
run:
	@echo "🚀 Running $(APP_NAME)..."
	@bash $(BUILD_DIR)/main.sh

# Clean the build directory
clean:
	@echo "🧹 Cleaning up..."
	@rm -rf $(BUILD_DIR)
	@echo "✅ Cleaned."

# Help command
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build     Build the project"
	@echo "  run       Run the app"
	@echo "  clean     Remove build output"
	@echo "  help      Show this help message"

.PHONY: all build run clean help
