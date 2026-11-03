# Spec 14: Remover Registry Duplicado e Consolidar Publicação no Registry Lumen

## 1. Contexto e Motivação
Na esteira de CI do `joaju-csv` (`.forgejo/workflows/ci.yml`), a etapa de configuração de credenciais do Cargo declara dois registries com a mesma URL index (`sparse+https://gambi.2lp.in/api/packages/lumen/cargo/`), nomeando-os simultaneamente como `gambi` e `lumen`.
Quando o Cargo executa `cargo publish --registry lumen`, ele detecta essa redundância e falha com o erro:
`error: multiple registries are configured with the same index url 'sparse+https://gambi.2lp.in/api/packages/lumen/cargo/': gambi, lumen`.

Para garantir a publicação contínua e estável da crate, a configuração deve consolidar exclusivamente o registry canônico `lumen`.

## 2. Requisitos Técnicos
- Remover a injeção do registry `gambi` em `.forgejo/workflows/ci.yml`.
- Manter exclusivamente a declaração e autenticação de `[registries.lumen]` com o índice `sparse+https://gambi.2lp.in/api/packages/lumen/cargo/`.
- Atualizar o teste de auditoria de CI (`tests/audit_gitops_cascade.sh`) para certificar que nenhum registry duplicado seja configurado no workflow.
- Preservar a notificação em cascata ao daemon Kombinat (`POST http://127.0.0.1:4040/api/cascade/dispatch`).

## 3. Plano de Testes (DD-TDD)
- Fase 4 (Red Test): Adicionar asserção em `tests/audit_gitops_cascade.sh` para proibir configuração duplicada de registries (`registries.gambi`). O teste deve falhar enquanto o workflow ainda contiver a entrada duplicada.
- Fase 5 (Green Implement): Ajustar `.forgejo/workflows/ci.yml` removendo a duplicação. O teste passa.
- Fase 6 (Refactor): Limpeza e revisão estrutural do workflow e testes.
- Fase 7 (Mutation): Validação de robustez das asserções.
- Fase 8 (Quality Gate): Linters, formatters e suíte completa verdes.
- Fase 9 (Close): Fechamento do ticket e merge.

## 4. Status do Ciclo DD-TDD
- **Fase Atual:** Fase 7 (Testes de asserção auditados contra regressão).
