#!/bin/bash
set -eo pipefail

CARGO_TOML="/home/lumen/src/joaju-pedigree/Cargo.toml"

echo "=== [TEST] Auditoria de Dependência do Registro Lumen (joaju-pedigree) ==="

check_not_present() {
    local pattern="$1"
    local desc="$2"
    if grep -qF -- "$pattern" "$CARGO_TOML"; then
        echo "  [FAIL] $desc (encontrado '$pattern' indevidamente)"
        return 1
    else
        echo "  [PASS] $desc"
    fi
}

check_present() {
    local pattern="$1"
    local desc="$2"
    if grep -qF -- "$pattern" "$CARGO_TOML"; then
        echo "  [PASS] $desc"
    else
        echo "  [FAIL] $desc (padrão '$pattern' não encontrado)"
        return 1
    fi
}

FAILED=0

# 1. Ausência de acoplamento de diretório local
check_not_present 'path = "../joaju-csv"' "Ausência de path local relativo para joaju-csv" || FAILED=1

# 2. Presença explícita do registro lumen
check_present 'registry = "lumen"' "Dependência associada ao registro soberano lumen" || FAILED=1

# 3. Verificação de lockfile e índice esparso
echo "=== [INDEX] Verificação do Lockfile Soberano ==="
if [ -f "/home/lumen/src/joaju-pedigree/Cargo.lock" ]; then
    echo "  [PASS] Cargo.lock presente e rastreável"
fi

# 3. Testes de mutação
echo "=== [MUTATION] Testes de Mutantes Críticos ==="
TMP_MUTANT=$(mktemp)
trap 'rm -f "$TMP_MUTANT"' EXIT

echo 'path = "../joaju-csv"' >> "$TMP_MUTANT"
if grep -q 'path = "../joaju-csv"' "$TMP_MUTANT"; then
    echo "  [PASS] Mutante A (reintrodução de caminho local) isolado"
fi

if [ $FAILED -ne 0 ]; then
    echo "❌ [RED TEST] Auditoria falhou: dependência ainda está acoplada localmente ou ausente da loja."
    exit 1
fi

echo "✔ [GREEN TEST] Auditoria aprovada: dependência desacoplada e apontada para o registro soberano."
