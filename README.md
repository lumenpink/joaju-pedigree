# joaju-pedigree

[![crates.io](https://img.shields.io/crates/v/joaju-pedigree.svg)](https://crates.io/crates/joaju-pedigree)
[![docs.rs](https://docs.rs/joaju-pedigree/badge.svg)](https://docs.rs/joaju-pedigree)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-2.0.0-emerald.svg)](https://gost.2lp.in)

Biblioteca Rust e motor FFI de alta performance para modelagem genealógica, cálculo de parentesco e renderização de heredogramas clínicos em conformidade com as diretrizes NSGC e ACMG.

---

## 🎯 Escopo & Missão

O `joaju-pedigree` oferece paridade funcional com o ecossistema `pedtools` do GNU R, viabilizando execução nativa em compiladores como Joaju, serviços web e CLIs com tempos de compilação e renderização sub-milissegundo.

- **Validação Biológica**: Integridade parental, sexagem e detecção de ciclos.
- **Cálculo de Parentesco**: Coeficientes de parentesco, matrizes genéticas e peeling order.
- **Layout & Renderização**: Posicionamento determinístico e geração vetorial SVG clínica.
- **Interoperabilidade**: Compatível com Linkage PED, CSV/TSV e JSON.

Consulte a especificação arquitetural detalhada em [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

---

## 🛠️ Instalação & Uso Rápido

```toml
[dependencies]
joaju-pedigree = "0.2"
```

```rust
use joaju_pedigree::core::Ped;

let ped = Ped::from_csv("id,fid,mid,sex\n1,0,0,1\n2,0,0,2\n3,1,2,1")?;
let svg = ped.to_svg_default()?;
```

---

Feito com dedicação • [Apache-2.0](LICENSE)
