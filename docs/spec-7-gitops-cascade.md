# Especificação Normativa — Ticket #7
## Pipeline GitOps de Publicação no Registry Lumen e Propagação em Cascata para Ecossistema

**Norma de Referência:** GOST v1.5.0 (Perfil Lumen — Seção 9)  
**Status:** Aprovado para Implementação  
**Data:** 2026-10-20  

---

### 1. Escopo e Objetivos

Esta especificação define o pipeline unificado de integração contínua (CI), publicação no registry soberano de crates Forgejo (`sparse+https://git.2lp.in/api/packages/lumen/cargo/`) e propagação em cascata GitOps para os consumidores do ecossistema: **Joaju Table (`joaju-table`)**, **Joaju Pedigree (`joaju-pedigree`)** e **Joaju (`joaju`)**:

1. **Recepção de Gatilhos:** Aceita `workflow_dispatch:`, `push` e `pull_request`.
2. **Validação de Qualidade Rust:** Checagem de formatação (`cargo fmt --check`), clippy zero warnings (`cargo clippy --all-targets -- -D warnings`), suíte completa de testes (`cargo test`) e auditoria GOST (`cccp check --strict`).
3. **Publicação Soberana:** Publica a crate `joaju-csv` no registry `lumen` utilizando `${{ secrets.LUMEN_REGISTRY_TOKEN }}`.
4. **Propagação a Jusante (Downstream Cascade):** Notifica automaticamente os repositórios dependentes (`joaju-table`, `joaju-pedigree` e `joaju`) via API Forgejo/Codeberg (`POST .../actions/workflows/ci.yaml/dispatches`).
