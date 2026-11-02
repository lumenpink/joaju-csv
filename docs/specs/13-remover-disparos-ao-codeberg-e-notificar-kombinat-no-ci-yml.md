# Spec 13: Remover Disparos ao Codeberg e Notificar Kombinat no ci.yml

## 1. Contexto e Motivação
A esteira do `joaju-csv` realizava disparos diretos ao Codeberg tentando atualizar esteiras downstream (`joaju-table`, `joaju-pedigree`, `joaju`). Esse mecanismo é legado e falho. O gerenciamento de propagação de versão em cascata agora é centralizado no daemon `kombinat` (`http://127.0.0.1:4040/api/cascade/dispatch`).

Além disso:
- A constante `VERSION` foi exportada no `src/lib.rs`.
- Termos imperialistas foram purgados do workflow.

## 2. Requisitos Técnicos
- Removido step do Codeberg e inserida chamada HTTP POST para o Kombinat (`POST http://127.0.0.1:4040/api/cascade/dispatch`).
- Atualizado `tests/audit_gitops_cascade.sh` para verificar a integração com o Kombinat.
- Adicionado teste de integração `tests/version_spec.rs` verificando a constante `VERSION`.
- Exportada `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` em `src/lib.rs`.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Teste comprovadamente falho verificando `VERSION` e script de auditoria de CI.
- Fase 5: Implementação funcional concluída.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict` com 100% de conformidade.

## 4. Status do Ciclo DD-TDD
- **Status:** Concluído com sucesso (Fases 1 a 9).
