# claude-list Makefile
# Run `make` or `make help` to see available commands

.PHONY: help build test check lint fmt clippy clean run release check-diff check-fix

# Default target - show help
help:
	@echo "claude-list - Claude Code 配置目录解析工具"
	@echo ""
	@echo "使用方法:"
	@echo "  make [命令]           执行指定命令 (默认: help)"
	@echo "  make help             显示此帮助信息"
	@echo ""
	@echo "验证命令:"
	@echo "  make check            运行所有检查 (fmt + clippy + test)"
	@echo "  make check-fix        修复所有检查问题并运行测试"
	@echo "  make test             运行单元测试"
	@echo "  make lint             运行代码检查 (fmt check + clippy)"
	@echo "  make clippy           运行 clippy 静态分析"
	@echo "  make fmt              检查代码格式"
	@echo ""
	@echo "构建命令:"
	@echo "  make build            编译项目 (debug)"
	@echo "  make build-release    编译项目 (release)"
	@echo "  make clean            清理构建产物"
	@echo ""
	@echo "运行命令:"
	@echo "  make run              运行 claude-list (使用 ~/.claude)"
	@echo "  make run-test         运行 claude-list (使用 tests/fixtures/.claude)"
	@echo ""
	@echo "开发命令:"
	@echo "  make check-diff       检查未提交的更改"
	@echo "  make fmt-fix          自动修复格式问题"

# Build debug
build:
	cargo build --verbose

# Build release
build-release:
	cargo build --release --verbose

# Clean
clean:
	cargo clean

# Run tests
test:
	cargo test --all-features --verbose

# Run clippy
clippy:
	cargo clippy --all-features -- -D warnings

# Check format
fmt:
	cargo fmt --check

# Fix format
fmt-fix:
	cargo fmt

# Fix all issues and run tests
check-fix: fmt-fix
	@cargo clippy --all-features --fix --allow-staged --allow-dirty 2>/dev/null || true
	@cargo test --all-features --verbose

# Lint: fmt check + clippy
lint: fmt clippy

# Check all
check: lint test

# Run the tool
run:
	cargo run

# Run with test fixtures
run-test:
	cargo run -- --config tests/fixtures/.claude

# Check uncommitted changes
check-diff:
	@echo "=== 未提交的更改 ==="
	@git status --short
	@echo ""
	@echo "=== 更改统计 ==="
	@git diff --stat HEAD
