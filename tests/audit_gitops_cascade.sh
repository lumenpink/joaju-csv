#!/usr/bin/env bash
# ==============================================================================
# tests/audit_gitops_cascade.sh
# Valida integracao de workflow GitOps e cascata em joaju-csv (Ticket #7 e #15)
# ==============================================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

WORKFLOW=".forgejo/workflows/ci.yml"

if [ -f ".forgejo/workflows/publish.yaml" ] || [ -f ".forgejo/workflows/publish.yml" ]; then
  echo "✘ [FALHA] publish legado para crates.io ainda presente!"
  exit 1
fi

if [ ! -f "$WORKFLOW" ]; then
  echo "✘ [FALHA] $WORKFLOW inexistente!"
  exit 1
fi

python3 -c "
try:
    import yaml
    yaml.safe_load(open('$WORKFLOW'))
except ImportError:
    pass
" || {
  echo "✘ [FALHA] $WORKFLOW possui sintaxe YAML invalida!"
  exit 1
}

if ! grep -q "workflow_dispatch:" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao declara 'workflow_dispatch:'!"
  exit 1
fi

if ! grep -q -E "sparse\+https://(gambi|git)\.2lp\.in/api/packages/lumen/cargo/" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao configura endpoint sparse do registry lumen!"
  exit 1
fi

if ! grep -q "cargo publish --registry lumen" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao publica no registry lumen!"
  exit 1
fi

if grep -q "registries\.gambi" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW contem registry duplicado registries.gambi!"
  exit 1
fi

if ! grep -q "rustup component add" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao adiciona componentes rustfmt/clippy!"
  exit 1
fi

if ! grep -q "CANONICAL_VERSION" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao sincroniza a versao canonica!"
  exit 1
fi

if grep -q "codeberg.org" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW ainda contem referencias ao Codeberg legado!"
  exit 1
fi

if ! grep -q 'token = "Bearer %s"' "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW nao utiliza autenticacao Bearer para o cargo!"
  exit 1
fi

if grep -q "4040" "$WORKFLOW"; then
  echo "✘ [FALHA] $WORKFLOW contem chamada legada a daemon na porta 4040 (agora orquestrado via webhook nativo Forgejo->Gossnab)!"
  exit 1
fi

echo "✔ [PASS] Auditoria GitOps cascade em joaju-csv validada com sucesso."
