# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Remoção de campo `permissions:` incompatível com Forgejo Actions (closes #20).

### Added
- Esteira GitOps unificada, publicação no registry lumen e cascata downstream para joaju (closes #19).
- Migração da dependência joaju-csv para o registro soberano lumen e publicação da crate v0.1.0 (closes #18).
- Renomeação canônica de package e repositório para `pedigree-rs` (`pedigree_rs`) em conformidade com o padrão GOST.
- Integração da dependência local `ssot-rs` para resolução de datasets.
- Suporte universal a todos os cabeçalhos e sinônimos originais do pedtools e Joaju (`individual`/`id`, `father`/`dadid`/`fid`/`pai`, `mother`/`momid`/`mid`/`mae`, `fenotipo`/`afetado`/`affected`).
- Parsing e extração nativa de diretivas `%% chave = valor` embutidas em blocos de texto/markdown.
- Sistema de legendas com paridade estrita aos símbolos do Joaju (`empty_m`, `empty_f`, `sq_half`, `circ_half`, `sq_solid`, `circ_solid`, `suspect`, `deceased`, `proband`).
- Atalho inteligente de auto-legenda (`auto_legend = true` / flag `-l` na CLI) sintetizando automaticamente apenas os fenótipos e traços presentes na família.
- Validação de integridade biológica e detecção de ciclos acíclicos em grafos genealógicos via 3-color DFS.
- Consultas completas de parentesco: `parents`, `children`, `spouses`, `siblings` e `ancestors`.
- Detecção automática de uniões consanguíneas via interseção de conjuntos de ancestrais comuns.
- Motor de layout genealógico determinístico por camadas de gerações ($X, Y$) e cálculo de centro de uniões.
- Renderizador vetorial SVG com símbolos clínicos NSGC/ACMG (homem, mulher, sexo desconhecido, probando, falecido, portador, afecções e linha dupla para uniões consanguíneas).
- Card de Legenda Vetorial integrado e parametrizável.
- CLI `pedigreers` para geração direta de SVG a partir de planilhas CSV.
- Suíte exaustiva de testes de paridade com 100% de cobertura nos casos de referência.
- Inicialização do repositório em estrita conformidade com o padrão GOST.
