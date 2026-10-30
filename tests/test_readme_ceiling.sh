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

# Teste de mutacao: se adicionarmos 1 linha, o teste deve falhar
TMP_MUTATION=$(mktemp)
cat "$README_FILE" > "$TMP_MUTATION"
echo "" >> "$TMP_MUTATION"
MUT_COUNT=$(wc -l < "$TMP_MUTATION")
if [ "$MUT_COUNT" -gt 50 ]; then
    echo "✅ OK: Sensibilidade confirmada (mutacao com $MUT_COUNT linhas rejeitada)."
else
    echo "❌ FALHOU: Mutacao nao detectada."
    rm -f "$TMP_MUTATION"
    exit 1
fi
rm -f "$TMP_MUTATION"
