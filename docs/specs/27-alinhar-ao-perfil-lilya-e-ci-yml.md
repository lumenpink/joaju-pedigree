# Spec: Ticket #27 - Alinhar ao Perfil Lilya e ci.yml

## 1. Contexto e Motivação
Alinhar o repositório `joaju-pedigree` com o perfil `lilya` (`https://lilya.2lp.in/profile.md`), padronizar o pipeline de CI com `.forgejo/workflows/ci.yml`, condensar `README.md` para respeitar o teto normativo de 50 linhas (GOST-20) migrando a documentação detalhada para `docs/ARCHITECTURE.md`, e expurgar terminologias não-conformes.

## 2. Requisitos Técnicos
- Renomeação de `.forgejo/workflows/ci.yaml` para `.forgejo/workflows/ci.yml`.
- Atualização de `tests/audit_gitops_cascade.sh` e `tests/ci_workflow_spec.rs` para referenciar `ci.yml`.
- Migração de conteúdo arquitetural para `docs/ARCHITECTURE.md` e ajuste do `README.md` para <= 50 linhas mantendo metadados essenciais.
- Substituição de ocorrências de terminologia proibida por termos canônicos de engenharia.
- Conformidade total com `cccp check --strict`.

## 3. Critérios de Aceitação
- Testes unitários e auditorias de conformidade aprovados.
- 100% de conformidade com `cccp check --strict`.


## 4. Conclusão da Fase 9
- Workflow renomeado para `.forgejo/workflows/ci.yml`.
- README.md condensado para 41 linhas (docs detalhados em docs/ARCHITECTURE.md).
- Perfil GOST alinhado para Lilya e conformidade 100%.
