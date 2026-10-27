# Especificação 23: Configurar Registry Gambi e Estabilizar Upload de Staticlib no CI

- **ID do Ticket:** #23
- **Status:** Concluído (DD-TDD) (closes #23)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
A esteira de CI do `joaju-pedigree` falhou durante a execução do job `check-and-test` devido ao erro:
`registry index was not found in any configuration: gambi`.
O script do CI configurava `[registries.lumen]`, mas as dependências da crate (`joaju-csv`) apontam para `registry = "gambi"`.
Além disso, o upload de artefatos estáticos precisa de `--http1.1` e `--retry 3` para prevenir abortos em transferências de pacotes grandes.

## 2. Requisitos Técnicos
1. Atualizar `.forgejo/workflows/ci.yaml`:
   - Configurar `[registries.gambi]` e `[registries.lumen]` em `$HOME/.cargo/config.toml` e `$HOME/.cargo/credentials.toml`.
   - Utilizar `cargo fmt --check` em vez de `cargo fmt -- --check`.
   - Adicionar `--http1.1 --retry 3` nas chamadas de `curl -T` para Forgejo Generic Packages.

2. Criar teste de validação de workflow em `tests/ci_workflow_spec.rs`.

## 3. Critérios de Aceitação
1. Teste de workflow validando a presença de `[registries.gambi]`.
2. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem avisos.
3. `cccp check --strict` 100% conforme.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #23
- [x] Fase 2 (Branch): `fix/23-configurar-registry-gambi-ci`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Teste red validando configuração do CI (`tests/ci_workflow_spec.rs`)
- [x] Fase 5 (Green Implement): Ajustes no arquivo `.forgejo/workflows/ci.yaml`
- [x] Fase 6 (Refactor): Limpeza de mensagens e parâmetros
- [x] Fase 7 (Mutation Test): Validação de caminhos de configuração
- [x] Fase 8 (Quality Gate): Testes e auditoria Gosplan 100%
- [x] Fase 9 (Finish & Close): Concluído (closes #23)
