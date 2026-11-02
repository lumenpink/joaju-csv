# Spec 13: Remover Disparos ao Codeberg e Notificar Kombinat no ci.yml

## 1. Contexto e Motivação
A esteira do `joaju-csv` realizava disparos diretos ao Codeberg tentando atualizar esteiras downstream (`joaju-table`, `joaju-pedigree`, `joaju`). Esse mecanismo é legado e falho. O gerenciamento de propagação de versão em cascata agora é centralizado no daemon `kombinat` (`http://127.0.0.1:4040/api/cascade/dispatch`).

Além disso:
- A constante `VERSION` deve ser exportada no `src/lib.rs`.
- Termos imperialistas devem ser purgados do workflow.

## 2. Requisitos Técnicos
- Remover step do Codeberg e inserir chamada HTTP POST para o Kombinat (`POST http://127.0.0.1:4040/api/cascade/dispatch`).
- Atualizar `tests/audit_gitops_cascade.sh` para verificar a integração com o Kombinat.
- Adicionar teste de integração `tests/version_spec.rs` verificando a constante `VERSION`.
- Exportar `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` em `src/lib.rs`.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Teste comprovadamente falho verificando `VERSION` e script de auditoria de CI.
- Fase 5: Implementação funcional.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict`.
