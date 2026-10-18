# joaju-pedigree

[![crates.io](https://img.shields.io/crates/v/joaju-pedigree.svg)](https://crates.io/crates/joaju-pedigree)
[![docs.rs](https://docs.rs/joaju-pedigree/badge.svg)](https://docs.rs/joaju-pedigree)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

A high-performance Rust library for genealogical modeling, kinship calculation, and clinical pedigree vector rendering conforming to **NSGC** (*National Society of Genetic Counselors*) and **ACMG** (*American College of Medical Genetics*) guidelines.

> ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
> This library is currently in early active development (`0.1.0`). It is NOT recommended for production use beyond experimental testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.

---

## 🎯 Scope & Mission

`joaju-pedigree` provides a **100% native Rust** implementation with functional parity to the established **`pedtools`** R ecosystem (developed by Magnus Dehli Vigeland / University of Oslo), removing runtime GNU R dependencies and enabling sub-millisecond pedigree rendering inside compilers like **Joaju**, web servers, and CLI tools.

---

## 🧬 Modules & Features (Parity with `pedtools`)

### 1. Structural Core (`core` / `ped`)
* **`Ped` and `Individual` Arena Structures**: Modeled with graph arenas (`petgraph` and stable identifiers) supporting inbreeding loops and multiple family nuclei safely.
* **Strict Biological Validation**:
  * Detection of impossible biological cycles (where an individual is their own ancestor).
  * Biological sex and parental consistency checks.
  * Automatic identification of founders and leaves.
* **Universal Format Parsers**:
  * CSV/TSV with standard and localized synonym resolution (`id, fid, mid, sex, status, proband, aff, ...`).
  * Linkage Pedigree Format (`.ped` and `.fam` from PLINK / Mendel).
  * JSON serialization and deserialization via `serde`.

### 2. Graph Theory & Kinship Calculations (`kinship` and `graph`)
* **Neighborhood Queries**:
  * `parents()`, `children()`, `spouses()`.
  * `siblings()`, `full_siblings()`, `half_siblings()`.
  * `ancestors()`, `descendants()`, `unrelated()`.
* **Lineage Metrics & Kinship Matrix**:
  * Kinship coefficients matrix computation (`KinshipMatrix`).
  * Inbreeding loop detection and resolution (`has_inbreeding_loops`, `break_loops`).
  * Peeling order algorithm for probabilistic genotype inference.

### 3. Genetic Markers & Mendelian Verification (`genetics`)
* Allelic markers and genotype definitions.
* Automatic Mendelian incompatibility validation (`mendelian_check`).

### 4. Geometric Layout Engine (`layout`)
* Deterministic grid positioning algorithm $(X, Y)$:
  * Strict horizontal generational layering ($Y$).
  * Horizontal ordering of spouses and sibling blocks to minimize edge crossings ($X$).
  * Support for multiple consorts and complex family arrangements.

### 5. Clinical Vector Rendering (`render`)
* **NSGC / ACMG Clinical Guidelines**:
  * Standard shapes: Square (male), Circle (female), Diamond (unspecified sex).
  * Proband: Directional arrow indicator with 'P' badge.
  * Deceased: Diagonal strike-through line.
  * Affection Status: Full fill or partitioned into up to 4 quadrants.
  * Carrier: Central dot or half-fill.
  * Twin status: Monozygotic and dizygotic connections.
  * Consanguineous mating: Double horizontal marriage line.
  * Integrated parametric vector legend card.

---

## 🚀 Installation

Add `joaju-pedigree` to your `Cargo.toml`:

```toml
[dependencies]
joaju-pedigree = "0.1.0"
```

Or install the CLI tool:

```bash
cargo install joaju-pedigree
```

---

## ⚡ Usage Example (Rust API)

```rust
use joaju_pedigree::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Declarative construction from CSV or canonical builders
    let ped = Ped::nuclear(2)?;
    assert_eq!(ped.len(), 4);

    // 2. Compute kinship matrix
    let kin = KinshipMatrix::compute(&ped);
    let phi_1_3 = kin.get("1", "3").unwrap();
    assert!((phi_1_3 - 0.25).abs() < 1e-6);

    // 3. Compute clinical layout
    let layout = ped.compute_layout(&LayoutOptions::default())?;

    // 4. Render to clinical SVG
    let svg = ped.render_svg(&layout, &RenderOptions::default());
    assert!(svg.contains("<svg"));

    Ok(())
}
```

---

## 💻 CLI Usage

```bash
# Render a pedigree to SVG
joaju-pedigree --input family.csv --output pedigree.svg --legend

# Validate Mendelian consistency
joaju-pedigree check-mendel data.ped

# Compute kinship coefficients
joaju-pedigree kinship family.csv
```

---

## 📋 GOST Compliance

This project adheres to the **[GOST](https://codeberg.org/lumenpink/gost)** specification (Gosplan Standard for Technical Software).
- **GOST Version:** `1.1.0`
- **Declared Languages:** `rust,shell`
- **License:** `Apache-2.0`

```bash
# Run strict compliance verification
cccp check --strict
```

---

## 📄 License

Distributed under the [Apache-2.0](./LICENSE) license.
