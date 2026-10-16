# pedigreers

> **pedigreers**: Motor soberano e biblioteca de alta performance em Rust para modelagem genealógica, cálculo de parentesco e renderização vetorial de heredogramas clínicos conformes com as diretrizes da **NSGC** (*National Society of Genetic Counselors*) e **ACMG** (*American College of Medical Genetics*).

---

## 🎯 Escopo & Missão

O `pedigreers` nasce para fornecer uma implementação **100% nativa em Rust** com paridade funcional ao consagrado ecossistema **`pedtools`** do R (desenvolvido por Magnus Dehli Vigeland / Universidade de Oslo), eliminando a dependência do runtime GNU R e permitindo a renderização instantânea de heredogramas em microssegundos dentro de compiladores como o **Joaju**, servidores web e ferramentas CLI.

---

## 🧬 Módulos do `pedigreers` (Paridade com `pedtools`)

### 1. Núcleo Estrutural (`core` / `ped`)
* **Estrutura `Ped` e `PedList`**: Modelagem baseada em Arena (`petgraph` e identificadores inteiros/UUID) para suporte seguro a ciclos consanguíneos e múltiplos núcleos familiares.
* **Validação Biológica Estrita**:
  * Detecção de ciclos biológicos impossíveis (onde um indivíduo é seu próprio ancestral).
  * Consistência de sexo biológico e relações parentais.
  * Identificação de fundadores (*founders*) e folhas (*leaves*).
* **Parsers de Formatos Universais**:
  * CSV/TSV (`id, pai, mae, sexo, status, probando, aff, ...`)
  * Linkage Pedigree Format (`.ped` e `.fam` do PLINK / Mendel).
  * Serialização e desserialização JSON via `serde`.

### 2. Teoria dos Grafos & Relações de Parentesco (`kinship`)
* **Consultas de Vizinhança**:
  * `parents()`, `children()`, `spouses()`.
  * `siblings()`, `full_siblings()`, `half_siblings()`.
  * `ancestors()`, `descendants()`, `unrelated()`.
* **Métricas de Linhagem**:
  * Cálculo de gerações e profundidade de árvore genealógica.
  * Detecção e isolamento de laços consanguíneos (*inbreeding loops*).

### 3. Mutadores de Linhagem (`modify`)
* `add_parents()`, `add_children()`, `add_spouse()`.
* `remove_individuals()`, `swap_sex()`, `relabel()`.
* Extração de sub-árvores e ramos (*branch extraction*).

### 4. Motor de Layout Geométrico (`layout`)
* Algoritmo de posicionamento determinístico em grade $(X, Y)$:
  * Alinhamento estrito em camadas horizontais por geração ($Y$).
  * Ordenação horizontal de casais e blocos de irmãos para minimização e prevenção de cruzamento de linhas em consanguinidade ($X$).
  * Suporte a múltiplos casamentos e conexões inter-geracionais.

### 5. Renderização Vetorial Clínica (`render`)
* **Diretrizes NSGC / ACMG**:
  * Formas padrão: Quadrado (homem), Círculo (mulher), Losango (sexo desconhecido).
  * Probando: Seta indicadora direcionada com letra identificadora 'P'.
  * Falecido: Linha diagonal de corte.
  * Status de Afecção: Preenchimento total ou particionado em quadrantes (1 a 4 condições simultâneas).
  * Portador (*Carrier*): Ponto central ou meio preenchimento.
  * Gemelaridade: Gêmeos monozigóticos (barra transversal) e dizigóticos (ramo triangular).
  * Casamento Consanguíneo: Linha dupla de união matrimonial.
* **Formatos de Saída**:
  * SVG vetorial puro e escalável.
  * Código Typst nativo (para inclusão in-process no Joaju).
  * Card de Legenda Vetorial parametrizável integrado.

---

## ⚡ Exemplo Rápido de Uso

```rust
use pedigreers::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Criação declarativa de linhagem
    let mut ped = Ped::from_csv_str(
        "id,pai,mae,sexo,status,probando,aff\n\
         1,0,0,1,0,0,0\n\
         2,0,0,2,0,0,0\n\
         3,1,2,1,0,1,1\n"
    )?;

    // 2. Cálculo de layout clínico
    let layout = ped.compute_layout(LayoutOptions::default())?;

    // 3. Renderização para SVG
    let svg = layout.to_svg(RenderOptions::clinical_default())?;
    std::fs::write("heredograma.svg", svg)?;

    Ok(())
}
```

---

## 📜 Licença

Distribuído sob a licença **Apache-2.0**.
