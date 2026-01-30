#!/bin/bash

# Quick Start Guide for NLP Chatbot

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         NLP Chatbot - Quick Start Guide                   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust is installed"
echo ""

# Build the project
echo "📦 Building the project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    echo ""
else
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

# Run tests
echo "🧪 Running tests..."
cargo test --quiet

if [ $? -eq 0 ]; then
    echo "✓ All tests passed!"
    echo ""
else
    echo "⚠️  Some tests failed, but you can still run the chatbot."
    echo ""
fi

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                  Available Commands                        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "  Run the chatbot:"
echo "    cargo run --release"
echo ""
echo "  Run tests:"
echo "    cargo test"
echo ""
echo "  Run examples:"
echo "    cargo run --example usage"
echo ""
echo "  Build documentation:"
echo "    cargo doc --open"
echo ""

# Ask if user wants to run the chatbot now
read -p "Would you like to start the chatbot now? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "🤖 Starting chatbot..."
    echo ""
    cargo run --release
fi
