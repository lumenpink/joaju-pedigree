# Especificação Técnica: Autenticação Soberana de Publicação e Falha Estrita

- **Ticket**: [#24](https://gambi.2lp.in/lumen/joaju-pedigree/issues/24)
- **Status**: Fase 3: Especificação
- **Data**: 2026-10-27
- **Autor**: Lumen Pink & Antigravity

---

## 1. Contexto e Motivação

O job de publicação no Forgejo falhava ao subir bibliotecas estáticas (`staticlib` `.a`) para o Generic Packages (`https://gambi.2lp.in/api/packages/lumen/generic/...`) com código HTTP 401 (`authGroup.Verify` / curl exit code 22).

A causa é que a secret `${{ secrets.FORGEJO_TOKEN }}` é um token efêmero gerado pelo runner com permissões restritas. Os tokens com permissão de escrita no registry e generic packages são os tokens de usuário `${{ secrets.REGISTRY_TOKEN }}` ou `${{ secrets.GAMBI_2LP_IN_TOKEN }}`.

Adicionalmente, a configuração simultânea de `[registries.gambi]` e `[registries.lumen]` com a mesma URL de índice gerava conflito no Cargo (`multiple registries are configured with the same index url`).

A diretiva do projeto exige **falha estrita** (`mas falhe se não conseguir publicar`): se a autenticação estiver ausente ou se a publicação falhar, o job de CI deve falhar explicitamente sem fallback mascarado (`|| true`).

---

## 2. Decisões de Arquitetura

1. **Cadeia Canônica de Secrets:**
   - Obter o token através de:
     `TOKEN="${{ secrets.REGISTRY_TOKEN || secrets.GAMBI_2LP_IN_TOKEN || secrets.FORGEJO_TOKEN }}"`
   - Validar a presença obrigatória do token:
     Se vazio, emitir erro e encerrar a pipeline imediatamente (`exit 1`).

2. **Unificação do Registro Cargo:**
   - Declarar exclusivamente `[registries.gambi]` com seu índice sparse, eliminando a colisão de múltiplos registros com a mesma URL.

3. **Falha Estrita em Publicações:**
   - `cargo publish --registry gambi --allow-dirty` sem fallback tolerante que oculte falhas reais de autenticação ou integridade.
   - Chamadas `curl -s -f --http1.1 --retry 3 -X PUT` para o Generic Packages devem interromper a pipeline se houver código `>= 400`.
   - Remoção de qualquer `|| true` tolerante no fallback de upload.
