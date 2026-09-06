#!/bin/bash
set -e

echo "=== RUNNING SYSTEM VERIFICATION PIPELINE ==="

echo ""
echo "[1/3] Running strict style and logic audits..."
cargo clippy --all-targets -- -D warnings

echo ""
echo "[2/3] Executing engine test suites..."
cargo test --release

echo ""
echo "[3/3] Compiling optimized low-level production binary..."
cargo build --release

echo ""
echo "========================================="
echo "   SYSTEM STATUS: SECURE & COMPILED      "
echo "========================================="
