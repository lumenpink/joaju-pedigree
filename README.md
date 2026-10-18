# pedigree-rs

[![crates.io](https://img.shields.io/crates/v/pedigree-rs.svg)](https://crates.io/crates/pedigree-rs)
[![docs.rs](https://docs.rs/pedigree-rs/badge.svg)](https://docs.rs/pedigree-rs)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

Motor soberano e biblioteca de alta performance em Rust para modelagem genealógica, cálculo de parentesco e renderização vetorial de heredogramas clínicos conformes com as diretrizes da **NSGC** (*National Society of Genetic Counselors*) e **ACMG** (*American College of Medical Genetics*).

> ⚠️ **Aviso de Versão em Desenvolvimento Ativo (pre-1.0 / dev)**  
> Este crate encontra-se em desenvolvimento inicial ativo (`0.1.x`). Mudanças na API pública e aperfeiçoamentos estruturais podem ocorrer antes da versão 1.0.0 estável. Feedback e contribuições são muito bem-vindos!

---

## 🎯 Escopo & Missão

O `pedigree-rs` nasce para fornecer uma implementação **100% nativa em Rust** com paridade funcional ao consagrado ecossistema **`pedtools`** do R (desenvolvido por Magnus Dehli Vigeland / Universidade de Oslo), eliminando a dependência do runtime GNU R e permitindo a renderização instantânea de heredogramas em microssegundos dentro de compiladores como o **Joaju**, servidores web e ferramentas CLI.

---

## 🧬 Módulos do `pedigree-rs` (Paridade com `pedtools`)

### 1. Núcleo Estrutural (`core` / `ped`)
* **Estrutura `Ped` e `Individual`**: Modelagem baseada em Arena (`petgraph` e identificadores) para suporte seguro a ciclos consanguíneos e múltiplos núcleos familiares.
* **Validação Biológica Estrita**:
  * Detecção de ciclos biológicos impossíveis (onde um indivíduo é seu próprio ancestral).
  * Consistência de sexo biológico e relações parentais.
  * Identificação de fundadores (*founders*) e folhas (*leaves*).
* **Parsers de Formatos Universais**:
  * CSV/TSV e sinônimos oficiais/PT (`id, pai, mae, sexo, status, probando, aff, ...`)
  * Linkage Pedigree Format (`.ped` e `.fam` do PLINK / Mendel).
  * Serialização e desserialização JSON via `serde`.

### 2. Teoria dos Grafos & Relações de Parentesco (`kinship` e `graph`)
* **Consultas de Vizinhança**:
  * `parents()`, `children()`, `spouses()`.
  * `siblings()`, `full_siblings()`, `half_siblings()`.
  * `ancestors()`, `descendants()`, `unrelated()`.
* **Métricas de Linhagem e Matriz de Parentesco**:
  * Cálculo da matriz de coeficientes de parentesco (`KinshipMatrix`).
  * Detecção e resolução de laços consanguíneos (`has_inbreeding_loops`, `break_loops`).
  * Algoritmo de peeling order para inferência probabilística genotípica.

### 3. Marcadores Genéticos e Verificação Mendeliana (`genetics`)
* Definição de marcadores alélicos e genótipos.
* Verificação automática de incompatibilidade mendeliana (`mendelian_check`).

### 4. Motor de Layout Geométrico (`layout`)
* Algoritmo de posicionamento determinístico em grade $(X, Y)$:
  * Alinhamento estrito em camadas horizontais por geração ($Y$).
  * Ordenação horizontal de casais e blocos de irmãos para minimização de cruzamento de linhas ($X$).
  * Suporte a múltiplos casamentos e conexões inter-geracionais.

### 5. Renderização Vetorial Clínica (`render`)
* **Diretrizes NSGC / ACMG**:
  * Formas padrão: Quadrado (homem), Círculo (mulher), Losango (sexo desconhecido).
  * Probando: Seta indicadora direcionada com letra 'P'.
  * Falecido: Linha diagonal de corte.
  * Status de Afecção: Preenchimento total ou particionado em até 4 quadrantes.
  * Portador (*Carrier*): Ponto central ou meio preenchimento.
  * Gemelaridade: Gêmeos monozigóticos e dizigóticos.
  * Casamento Consanguíneo: Linha dupla de união matrimonial.
  * Card de Legenda Vetorial parametrizável integrado.

---

## 🚀 Instalação

Adicione `pedigree-rs` ao seu `Cargo.toml`:

```toml
[dependencies]
pedigree-rs = "0.1.0-dev.1"
```

Ou instale a CLI:

```bash
cargo install pedigree-rs
```

---

## ⚡ Exemplo de Uso (Rust API)

```rust
use pedigree_rs::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Criação declarativa a partir de CSV ou builders típicos
    let ped = Ped::nuclear(2)?;
    assert_eq!(ped.len(), 4);

    // 2. Matriz de parentesco
    let kin = KinshipMatrix::compute(&ped);
    let phi_1_3 = kin.get("1", "3").unwrap();
    assert!((phi_1_3 - 0.25).abs() < 1e-6);

    // 3. Cálculo de layout clínico
    let layout = PedLayout::compute(&ped, &LayoutOptions::default())?;

    // 4. Renderização para SVG vetorial
    let svg = render_svg(&ped, &layout, &RenderOptions::default());
    assert!(svg.contains("<svg"));

    Ok(())
}
```

---

## 💻 Uso via CLI

```bash
# Renderizar heredograma para SVG
pedigree-rs plot familia.csv -o heredograma.svg --legend

# Validar herança mendeliana
pedigree-rs check-mendel dados.ped

# Calcular coeficientes de parentesco
pedigree-rs kinship familia.csv
```

---

## 📋 Conformidade GOST

Este projeto adota a norma **[GOST](https://codeberg.org/lumenpink/gost)** (Gosplan Standard for Technical Software).
- **Versão GOST:** `1.1.0`
- **Linguagens declaradas:** `rust,shell`
- **Licença:** `Apache-2.0`

```bash
# Executar auditoria de qualidade estrita
cccp check --strict
```

---

## 📄 Licença

Distribuído sob a licença [Apache-2.0](./LICENSE).
