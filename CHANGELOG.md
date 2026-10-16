# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Parser universal de CSV com normalização de termos em português e inglês (`pai`/`fid`, `mae`/`mid`, `sexo`/`sex`, etc.).
- Validação de integridade biológica e detecção de ciclos acíclicos em grafos genealógicos via 3-color DFS.
- Consultas completas de parentesco: `parents`, `children`, `spouses`, `siblings` e `ancestors`.
- Detecção automática de uniões consanguíneas via interseção de conjuntos de ancestrais comuns.
- Motor de layout genealógico determinístico por camadas de gerações ($X, Y$) e cálculo de centro de uniões.
- Renderizador vetorial SVG com símbolos clínicos NSGC/ACMG (homem, mulher, sexo desconhecido, probando, falecido, portador, afecções e linha dupla para uniões consanguíneas).
- Card de Legenda Vetorial integrado e parametrizável.
- CLI `pedigreers` para geração direta de SVG a partir de planilhas CSV.
- Suíte exaustiva de testes de paridade com 100% de cobertura nos casos de referência.
- Inicialização do repositório em estrita conformidade com o padrão GOST.
