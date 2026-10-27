# Especificação Normativa — Ticket #25
## Atualização da Auditoria de Cascade GitOps e Idempotência Estrita de Publicação

**Norma de Referência:** GOST v1.5.0 (Perfil Lumen — Seção 9)  
**Status:** Aprovado para Implementação  
**Data:** 2026-10-27  

---

### 1. Escopo e Objetivos

Esta especificação formaliza a correção da auditoria de CI e a política estrita de publicação da crate `joaju-pedigree` e seus binários estáticos:

1. **Conformidade de Nomes de Registry:** O script `tests/audit_gitops_cascade.sh` deve validar que o workflow publica no registry soberano, aceitando tanto `gambi` quanto `lumen` (`cargo publish --registry (lumen|gambi)`).
2. **Idempotência Estrita no Cargo Publish:** O passo de publicação deve executar `cargo publish --registry gambi --allow-dirty`. Se o comando falhar:
   - Se o erro for estritamente de versão já existente no índice (`already exists`), deve registrar um aviso informativo e concluir com sucesso.
   - Qualquer outro erro (credenciais, token expirado, erro 401, timeout, falha de rede ou compilação) deve resultar em falha estrita do pipeline (`exit 1`), garantindo que não haja mascaramento indevido de erros de publicação.
3. **Preservação do Upload Estrito de Generic Packages:** A publicação das bibliotecas estáticas (`libjoaju_pedigree-*.a`) deve continuar estrita via `curl -s -f --http1.1 --retry 3`, abortando com erro em caso de qualquer código HTTP de falha (4xx/5xx).
