# Spec 30: Remover Disparos ao Codeberg e Notificar Kombinat no ci.yml

## 1. Contexto e Motivação
A esteira do `joaju-pedigree` continha um step legado que realizava requisições `curl` para a API do Codeberg tentando disparar a CI downstream do `joaju`. Esse mecanismo é obsoleto e frágil. Com o daemon `kombinat` gerenciando a esteira em cascata, esse step legado deve ser inteiramente expurgado e substituído por uma notificação direta ao Kombinat em `http://127.0.0.1:4040/api/cascade/dispatch`.

Além disso, eliminam-se quaisquer termos contrários às diretrizes de estilo do projeto.

## 2. Requisitos Técnicos
- Remover o step `GitOps Cascade — Dispatch Downstream joaju` que acessava o Codeberg.
- Adicionar o step de notificação ao daemon Kombinat após a publicação dos pacotes genéricos e da crate.
- Auditar no teste automatizado `tests/ci_workflow_spec.rs` que referências ao Codeberg foram extirpadas e que o Kombinat é acionado.
- Atualizar script de auditoria `tests/audit_gitops_cascade.sh`.

## 3. Plano de Testes (DD-TDD)
- Fase 4 (Red Test): Adicionadas asserções em `tests/ci_workflow_spec.rs` exigindo `http://127.0.0.1:4040/api/cascade/dispatch` e banindo `codeberg.org`.
- Fase 5 (Green Implement): Atualizados `.forgejo/workflows/ci.yml` e `tests/audit_gitops_cascade.sh`.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict` com 100% de conformidade.

## 4. Status do Ciclo DD-TDD
- **Status:** Concluído com sucesso (Fases 1 a 9).
