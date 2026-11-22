# Especificação Normativa — Ticket #7
## Pipeline GitOps de Publicação no Registry Lumen e Propagação em Cascata para Ecossistema

**Norma de Referência:** GOST v3.0.0 (Perfil Lilya 2.1)  
**Status:** Aprovado para Implementação  
**Data:** 2026-10-20 (Atualizado 2026-11-22)  

---

### 1. Escopo e Objetivos

Esta especificação define o pipeline unificado de integração contínua (CI), publicação no registry autônomo de crates Forgejo (`sparse+https://gambi.2lp.in/api/packages/lumen/cargo/`) e propagação em cascata GitOps para os consumidores do ecossistema: **Joaju Table (`joaju-table`)**, **Joaju Pedigree (`joaju-pedigree`)** e **Joaju (`joaju`)**:

1. **Recepção de Gatilhos:** Aceita `workflow_dispatch:`, `push` e `pull_request`.
2. **Validação de Qualidade Rust:** Checagem de formatação determinística (`cargo fmt --check`), clippy zero warnings (`cargo clippy --all-targets -- -D warnings`), suíte completa de testes (`cargo test`) e auditoria GOST (`cccp check --strict`).
3. **Publicação Canônica:** Publica a crate `joaju-csv` no registry `lumen` utilizando `${{ secrets.LUMEN_REGISTRY_TOKEN }}`.
4. **Propagação a Jusante (Downstream Cascade):** A publicação de novo pacote no Forgejo emite webhook nativo de `package` para o Gossnab (`https://gossnab.lohn.in/api/v1/webhooks/forgejo`), que aciona `kn cascade --kind cargo` para propagar automaticamente as dependências do ecossistema.
