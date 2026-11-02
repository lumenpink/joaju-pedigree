# Spec 30: Remover Disparos ao Codeberg e Notificar Kombinat no ci.yml

## 1. Contexto e Motivação
A esteira do `joaju-pedigree` continha um step legado que realizava requisições `curl` para a API do Codeberg tentando disparar a CI downstream do `joaju`. Esse mecanismo é obsoleto e frágil. Com o daemon `kombinat` gerenciando a esteira em cascata, esse step legado deve ser inteiramente expurgado e substituído por uma notificação direta ao Kombinat em `http://127.0.0.1:4040/api/cascade/dispatch`.

Além disso, eliminam-se quaisquer termos imperialistas em desacordo com as diretrizes do projeto.

## 2. Requisitos Técnicos
- Remover o step `GitOps Cascade — Dispatch Downstream joaju` que acessava o Codeberg.
- Adicionar o step de notificação ao daemon Kombinat após a publicação dos pacotes genéricos e da crate.
- Auditar no teste automatizado `tests/ci_workflow_spec.rs` que referências ao Codeberg foram extirpadas e que o Kombinat é acionado.

## 3. Plano de Testes (DD-TDD)
- Fase 4 (Red Test): Adicionar asserções em `tests/ci_workflow_spec.rs` exigindo `http://127.0.0.1:4040/api/cascade/dispatch` e banindo `codeberg.org`.
- Fase 5 (Green Implement): Atualizar `.forgejo/workflows/ci.yml`.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict`.
