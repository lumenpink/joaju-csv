#!/usr/bin/env bash
set -euo pipefail

README_FILE="README.md"
if [ ! -f "$README_FILE" ]; then
    echo "❌ FALHOU: $README_FILE ausente."
    exit 1
fi

LINE_COUNT=$(wc -l < "$README_FILE")
if [ "$LINE_COUNT" -gt 50 ]; then
    echo "❌ FALHOU: README.md excede o teto normativo de 50 linhas (possui $LINE_COUNT linhas)"
    exit 1
fi

echo "✅ OK: README.md possui $LINE_COUNT linhas (teto <= 50)."
