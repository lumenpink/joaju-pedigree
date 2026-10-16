#!/usr/bin/env bash
set -euo pipefail

# Diretório base
FIXTURES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$FIXTURES_DIR/../.." && pwd)"
R_GEN_DIR="$FIXTURES_DIR/r_generated"
PED_GEN_DIR="$FIXTURES_DIR/pedigreers_generated"

mkdir -p "$R_GEN_DIR" "$PED_GEN_DIR"

CASES=(
    "g2_2gen"
    "g3_3gen"
    "g4_4gen"
    "g5_5gen"
    "half_sibs"
    "avuncular"
    "double_cousins"
    "half_sib_triangle"
    "full_sib_mating"
    "twins"
    "litter10"
)

echo "=== [1/2] Gerando referências no R via pedtools / kinship2 ==="
for c in "${CASES[@]}"; do
    csv_file="$FIXTURES_DIR/$c.csv"
    r_out="$R_GEN_DIR/${c}_pedtools_ref.svg"
    echo "  -> R: $csv_file => $r_out"
    Rscript /home/lumen/src/joaju/plot_pedtools.R "$csv_file" "$r_out" "list(cex = 0.9, symbolsize = 1.0)"
done

echo "=== [2/2] Gerando equivalentes no pedigreers (Rust) ==="
cd "$REPO_DIR"
cargo build --quiet

for c in "${CASES[@]}"; do
    csv_file="$FIXTURES_DIR/$c.csv"
    ped_out="$PED_GEN_DIR/${c}_pedigreers.svg"
    echo "  -> pedigreers: $csv_file => $ped_out"
    "$REPO_DIR/target/debug/pedigreers" -i "$csv_file" -o "$ped_out" -l
done

echo "=== Concluído com sucesso! 11 fixtures geradas em $R_GEN_DIR e $PED_GEN_DIR ==="
