# Spec #28 — Publicar Staticlibs sob Alias Latest no Generic Packages

## 1. Contexto e Objetivo
Garantir que a esteira CI do repositório `joaju-pedigree` publique as bibliotecas estáticas multiarquitetura (`libjoaju_pedigree-x86_64-linux.a`, `libjoaju_pedigree-aarch64-linux.a`, `libjoaju_pedigree-x86_64-windows.a`) não apenas na tag canônica `${VERSION}`, mas também sob o alias `latest` em Forgejo Generic Packages.

## 2. Requisitos
1. No job de compilação/publicação de staticlibs em `.forgejo/workflows/ci.yml`, para cada destino `${DEST_NAME}`, realizar o upload tanto para `${VERSION}` quanto para `latest`.
2. Tratar HTTP 201, 200 e 409 (conflito caso já exista o pacote) como códigos de sucesso idempotentes no upload para `latest`.
3. Adicionar asserções em teste de workflow (`tests/ci_workflow_spec.rs`) validando a presença da publicação sob `latest`.

## 3. Plano de Ação (GOST DD-TDD)
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho validando presença da publicação sob `latest` em `tests/ci_workflow_spec.rs`.
- Fase 5: Implementação verde em `.forgejo/workflows/ci.yml`.
- Fase 6: Refatoração e limpeza.
- Fase 7: Teste de mutação.
- Fase 8: Quality Gate (`cargo fmt`, `cargo clippy`, `cargo test`, `cccp check --strict`).
- Fase 9: Conclusão, merge e push.
