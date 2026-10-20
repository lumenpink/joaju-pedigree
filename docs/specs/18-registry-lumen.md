# Especificação Normativa — Ticket #18
## Migração da Dependência joaju-csv para o Registro Soberano Lumen e Publicação

**Norma de Referência:** GOST v1.5.0 (Perfil Lumen — Seção 9)  
**Status:** Aprovado para Implementação  
**Data:** 2026-10-20  

---

### 1. Escopo e Objetivos

Esta especificação define a transição arquitetural da crate **`joaju-pedigree`** para consumir dependências através do registro soberano de pacotes Cargo no Forgejo central (`https://git.2lp.in/api/packages/lumen/cargo/`):

1. **Desacoplamento de Caminho Local:**
   - Remoção de `path = "../joaju-csv"` no `Cargo.toml`.
   - Declaração explícita da procedência da dependência:
     ```toml
     joaju-csv = { version = "0.1.0", registry = "lumen" }
     ```
2. **Resolução Hermética de Dependências:**
   - Verificação de compilação sem necessidade de diretórios irmãos locais (`../joaju-csv`).
3. **Publicação no Registro Soberano:**
   - Publicação da versão `0.1.0` da crate `joaju-pedigree` no registro `lumen`.
