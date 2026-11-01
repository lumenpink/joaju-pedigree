# Joaju Pedigree — Arquitetura e Especificações Detalhadas

## 🎯 Escopo & Missão
`joaju-pedigree` fornece uma implementação 100% nativa em Rust com paridade funcional ao ecossistema `pedtools` (desenvolvido por Magnus Dehli Vigeland / Universidade de Oslo), eliminando dependências de runtime do GNU R e viabilizando renderização vetorial sub-milissegundo de heredogramas clínicos dentro de compiladores como Joaju, servidores web e ferramentas CLI.

---

## 🧬 Módulos & Recursos (Paridade com `pedtools`)

### 1. Structural Core (`core` / `ped`)
* **Estruturas de Arena `Ped` e `Individual`**: Modeladas com arenas de grafos (`petgraph` e identificadores estáveis), suportando loops de endocruzamento e múltiplos núcleos familiares.
* **Validação Biológica Estrita**:
  * Detecção de ciclos biológicos impossíveis (onde um indivíduo é seu próprio ancestral).
  * Verificação de sexo biológico e consistência parental.
  * Identificação automática de fundadores e folhas.
* **Parsers de Formatos Universais**:
  * CSV/TSV com resolução de sinônimos padronizados e localizados (`id, fid, mid, sex, status, proband, aff, ...`).
  * Linkage Pedigree Format (`.ped` e `.fam` do PLINK / Mendel).
  * Serialização e desserialização JSON via `serde`.

### 2. Teoria dos Grafos & Cálculos de Parentesco (`kinship` e `graph`)
* **Consultas de Vizinhança**:
  * `parents()`, `children()`, `spouses()`.
  * `siblings()`, `full_siblings()`, `half_siblings()`.
  * `ancestors()`, `descendants()`, `unrelated()`.
* **Métricas de Linhagem & Matriz de Parentesco**:
  * Cálculo da matriz de coeficientes de parentesco (`KinshipMatrix`).
  * Detecção e resolução de loops de endocruzamento (`has_inbreeding_loops`, `break_loops`).
  * Algoritmo de peeling order para inferência probabilística de genótipos.

### 3. Marcadores Genéticos & Verificação Mendeliana (`genetics`)
* Marcadores alélicos e definições de genótipo.
* Validação automática de incompatibilidade mendeliana (`mendelian_check`).

### 4. Motor de Layout Geométrico (`layout`)
* Algoritmo determinístico de posicionamento em grade $(X, Y)$:
  * Estratificação geracional horizontal estrita ($Y$).
  * Agrupamento conjugal e centralização balanceada de prole ($X$).
  * Roteamento ortogonal de conexões parentais sem cruzamentos indesejados.

### 5. Renderização Vetorial SVG (`draw` / `svg`)
* Renderização direta em SVG segundo diretrizes clínicas NSGC/ACMG:
  * Quadrados (masculino), círculos (feminino), losangos (sexo indefinido).
  * Preenchimentos para indivíduos afetados, transportadores e carreadores assintomáticos.
  * Probandos identificados com seta diagnóstica (`P`).
  * Linha diagonal para indivíduos falecidos.
  * Linhas duplas para casamentos consanguíneos.
