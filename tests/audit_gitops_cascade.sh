#!/usr/bin/env bash
# ==============================================================================
# tests/audit_gitops_cascade.sh
# Valida integracao de workflow GitOps e cascata em joaju-csv (Ticket #7)
# ==============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

WORKFLOW=".forgejo/workflows/ci.yaml"

if [ -f ".forgejo/workflows/publish.yaml" ]; then
  echo "✘ [FALHA] publish.yaml legado para crates.io ainda presente!"
  exit 1
fi

if [ ! -f "$WORKFLOW" ]; then
  echo "✘ [FALHA] $WORKFLOW inexistente!"
  exit 1
fi

if ! grep -q "workflow_dispatch:" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao declara 'workflow_dispatch:'!"
  exit 1
fi

if ! grep -q "sparse+https://git.2lp.in/api/packages/lumen/cargo/" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao configura endpoint sparse do registry lumen!"
  exit 1
fi

if ! grep -q "cargo publish --registry lumen" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao publica no registry soberano lumen!"
  exit 1
fi

if ! grep -q 'repos/lumenpink/\$target/actions/workflows/ci.yaml/dispatches' "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao dispara as esteiras downstream!"
  exit 1
fi

if ! grep -q 'joaju-table joaju-pedigree joaju' "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao lista todos os alvos downstream (joaju-table, joaju-pedigree, joaju)!"
  exit 1
fi

echo "✔ [PASS] Auditoria GitOps cascade em joaju-csv validada com sucesso."
