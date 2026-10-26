# Especificação 21: Suporte a Resolução Tabular SSROT e render_pedigree

- **ID do Ticket:** #21
- **Status:** Concluído (Fase 9)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Escopo
Incorporação da resolução de diretivas tabulares SSROT (`src`, `file`, `select`, `where`) com a crate `joaju-csv` e a função pública de alto nível `render_pedigree` diretamente na biblioteca `joaju-pedigree`.

## 2. Implementação
- Módulo `src/block.rs` contendo `render_pedigree`, `RenderRequest`, `RenderResponse`
- Suporte a `rlib` e `staticlib` em `[lib]`
- Dependência `joaju-csv` apontada para `registry = "gambi"`
- Versão elevada para `0.2.1` e publicada no registro soberano `gambi`
