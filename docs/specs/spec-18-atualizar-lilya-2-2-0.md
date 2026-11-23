# Spec #18: Atualizar Perfil Lilya para Versão 2.2.0

## 1. Contexto e Motivação
Com o lançamento do GOST Lilya 2.2.0 e Kollontai 2.2.0, o compilador Rust de referência foi atualizado para rustc >= 1.100.0 e as rotas canônicas de regras foram unificadas. O repositório joaju-csv precisa atualizar suas definições de governança (.gost/GOST.md e .gost/rules.toml) para herdar as regras e padrões da versão 2.2.0.

## 2. Escopo
- Atualizar referências de `2.1.0` para `2.2.0` em `.gost/GOST.md` e `.gost/rules.toml`.
- Atualizar a versão mínima do compilador Rust para `1.100.0`.
- Validar conformidade estrita com `cccp check --strict`.
- Validar integridade da crate com `cargo test`.

## 3. Plano de Implementação (GOST DD-TDD)
1. Fase 3: Especificação técnica (este documento).
2. Fase 4: Teste Red (N/A - atualização de metanorma/governança).
3. Fase 5: Ajuste de `.gost/GOST.md` e `.gost/rules.toml`.
4. Fase 6-7: Refatoração / Testes de Mutação (N/A para governança).
5. Fase 8: Quality Gate (`cargo test` e `cccp check --strict`).
6. Fase 9: Fechamento do ticket e sincronização.

## 4. Árvore de Decisão e Guardiã da Natasha
- **Por que manter .gost/GOST.md como SSOT?** A Natasha precisa de rastreabilidade direta do padrão adotado sem depender de ferramentas externas legadas. A especificação compilada localmente garante auditoria offline e determinística.
- **Alternativa descartada:** Manter 2.1.0 geraria divergência em relação ao ambiente de build do CI (`gost-ci:1.100.0`).

## 5. Resultados e Validação
- cccp check: 24/24 PASS (100% conformidade com Metanorma 2.2.0, Kollontai e Lilya).
- cargo test: 14 testes executados com sucesso (10 unit/integration, 3 version_spec, 1 doc-test).
- Status final: Aprovado para merge na branch principal.
