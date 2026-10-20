# Especificação Normativa — Ticket #19
## Pipeline GitOps de Publicação no Registry Lumen e Propagação em Cascata para Joaju

**Norma de Referência:** GOST v1.5.0 (Perfil Lumen — Seção 9)  
**Status:** Aprovado para Implementação  
**Data:** 2026-10-20  

---

### 1. Escopo e Objetivos

Esta especificação define o pipeline unificado de integração contínua (CI), publicação no registry soberano de crates Forgejo (`sparse+https://git.2lp.in/api/packages/lumen/cargo/`) e propagação em cascata GitOps para o compilador **Joaju (`joaju`)**:

1. **Recepção de Workflow Dispatch:** Aceita `workflow_dispatch:`, `push` e `pull_request`.
2. **Auto-atualização GitOps de Dependências Upstream:** Quando acionado via `workflow_dispatch`, executa `cargo update -p joaju-csv`. Se houver alteração em `Cargo.lock`, realiza commit e push para a branch `main`.
3. **Validação de Qualidade Rust:** Checagem de formatação (`cargo fmt --check`), clippy zero warnings (`cargo clippy --all-targets -- -D warnings`), suíte completa de testes (`cargo test`) e auditoria GOST (`cccp check --strict`).
4. **Publicação Soberana:** Publica a crate `joaju-pedigree` no registry `lumen` utilizando `${{ secrets.LUMEN_REGISTRY_TOKEN }}`.
5. **Propagação a Jusante (Downstream Cascade):** Notifica automaticamente o repositório `joaju` via API Forgejo/Codeberg (`POST .../actions/workflows/ci.yaml/dispatches`).
