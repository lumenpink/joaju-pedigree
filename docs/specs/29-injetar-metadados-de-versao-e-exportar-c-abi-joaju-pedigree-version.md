# Spec 29: Injetar Metadados de Versão e Exportar C-ABI joaju_pedigree_version

## 1. Contexto e Motivação
Para que o orquestrador `kombinat` e o binário unificado `joaju` possam diagnosticar e validar a versão exata do subsistema de heredogramas clínicos em runtime, a crate `joaju-pedigree` deve exportar:
1. `pub const VERSION: &str` com a versão canônica lida em tempo de compilação.
2. `#[no_mangle] pub extern "C" fn joaju_pedigree_version() -> *const c_char` retornando ponteiro estático terminado em nulo (`\0`).

## 2. Requisitos Técnicos
- `VERSION` exportada na raiz do crate (`src/lib.rs`).
- `joaju_pedigree_version()` em `src/ffi.rs` com linkage C (`#[no_mangle]`, `extern "C"`), apontando para literal estático sem alocação em heap.
- Testes de integração em `tests/version_spec.rs` cobrindo a constante e a chamada C-ABI.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Teste comprovadamente falho verificando `VERSION` e `joaju_pedigree_version()`.
- Fase 5: Implementação funcional.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict` com 100% de conformidade.

## 4. Status do Ciclo DD-TDD
- **Status:** Concluído com sucesso (Fases 1 a 9).
