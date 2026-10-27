# Especificação 22: Habilitar C-ABI FFI e Esteira de Pacotes Estáticos

- **ID do Ticket:** #22
- **Status:** Em Progresso (DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.1.0 (Rust)

## 1. Contexto & Motivação
A crate `joaju-pedigree` implementa modelagem de pedigrees clínicos e renderização de heredogramas em SVG compatível com diretrizes NSGC/ACMG e pedtools. Para viabilizar a distribuição e o consumo por bibliotecas estáticas externas sem custo de recompilação do grafo genético e das dependências tabulares (`joaju-csv`, `petgraph`), a `staticlib` (`libjoaju_pedigree.a` e `joaju_pedigree.lib`) deve expor uma interface estável de C-ABI.

## 2. Requisitos Técnicos
1. **Interface C-ABI (FFI):**
   - Função `joaju_pedigree_render`:
     ```c
     int32_t joaju_pedigree_render(
         const char* raw_code,
         const char* forced_format,
         char** out_format,
         char** out_svg,
         char** out_error
     );
     ```
     Retorna `0` em sucesso e popula `out_format` e `out_svg`. Retorna `-1` em caso de erro e popula `out_error`.
   - Função `joaju_pedigree_free_string(char* ptr)` para liberar memória de strings alocadas pela FFI.

2. **Garantia de Compilação `staticlib`:**
   - `crate-type = ["rlib", "staticlib"]` declarado em `Cargo.toml`.
   - Teste de integração de FFI em `tests/ffi_spec.rs`.

3. **Publicação Soberana:**
   - Upload do artefato estático `libjoaju_pedigree.a` para o registro genérico da Forgejo: `https://gambi.2lp.in/api/packages/lumen/generic/joaju-pedigree-static/...`.

## 3. Critérios de Aceitação
1. Funções C-ABI exportadas e validadas em `tests/ffi_spec.rs`.
2. Símbolos `joaju_pedigree_render` e `joaju_pedigree_free_string` presentes no arquivo `.a`.
3. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem avisos.
4. `cccp check --strict` 100% conforme.

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #22
- [x] Fase 2 (Branch): `feat/22-habilitar-c-abi-staticlib`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [ ] Fase 4 (Red Test): Criar teste red para símbolos C-ABI (`tests/ffi_spec.rs`)
- [ ] Fase 5 (Green Implement): Implementar módulo FFI (`src/ffi.rs`)
- [ ] Fase 6 (Refactor): Limpeza de memória e testes para ponteiros nulos
- [ ] Fase 7 (Mutation Test): Validação de mutação e resiliência em erros
- [ ] Fase 8 (Quality Gate): fmt, clippy sem avisos e testes verdes (100% Gosplan)
- [ ] Fase 9 (Finish & Close): Concluído e pronto para merge
