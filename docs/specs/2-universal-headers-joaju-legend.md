# Especificação Técnica: Headers Universais e Legendas com Paridade Joaju

- **Ticket:** #2
- **Branch:** `feat/2-universal-headers-joaju-legend`
- **Norma:** GOST DD-TDD v1.1.0
- **Status:** Concluído

---

## 1. Visão Geral

Esta especificação garante 100% de retrocompatibilidade com o comportamento do script R `plot_pedtools.R` do ecossistema Joaju, viabilizando:
1. Suporte a todas as colunas, sinônimos e valores em português e inglês.
2. Reconhecimento de diretivas no formato `%% chave = valor`.
3. Renderização de legendas idênticas à implementação do Joaju (mesmos símbolos e dimensões).
4. Atalho inteligente para auto-geração de legenda (`auto_legend = true` / flag `-l`).

---

## 2. Dicionário de Headers e Sinônimos

| Campo | Sinônimos Aceitos | Valores Mapeados |
| :--- | :--- | :--- |
| **ID** | `id`, `individual` | String única |
| **Pai** | `fid`, `pai`, `dadid`, `father` | String ou `0`, `""`, `NA` -> `None` |
| **Mãe** | `mid`, `mae`, `momid`, `mother` | String ou `0`, `""`, `NA` -> `None` |
| **Sexo** | `sex`, `sexo` | `1`, `M`, `MALE`, `MAS` -> Masculino;<br>`2`, `F`, `FEMALE`, `FEM` -> Feminino;<br>Outros -> Desconhecido |
| **Falecido** | `status`, `falecido`, `deceased` | `1`, `falecido`, `dead`, `deceased`, `sim`, `s`, `true`, `d` -> `Deceased` |
| **Probando** | `proband`, `probando`, `probanda` | `1`, `sim`, `true`, `s`, `proband`, `probanda`, `probando`, `p` -> `true` |
| **Fenótipo** | `fenotipo`, `afetado`, `affected`, `aff` | • `hom`, `homozigoto`, `afetado`, `doente`, `mut`, `affected`, `yes`, `sim`, `s`, `1` -> `affected = 1`<br>• `het`, `heterozigoto`, `portador`, `carrier` -> `carrier = true`<br>• `?`, `suspeita`, `suspect`, `duvida` -> `suspect = true` |
| **Portador** | `carrier`, `portador` | `1`, `sim`, `s`, `true` -> `carrier = true` |
| **Suspeito** | `suspect`, `suspeito` | `1`, `sim`, `s`, `true` -> `suspect = true` |

---

## 3. Diretivas e Sistema de Legenda

### 3.1 Diretivas `%%`
Linhas iniciadas com `%%` são separadas antes do parse tabular. São suportadas:
* `%% legend = c("Homem", "Mulher", ...)` ou `%% legend = auto`
* `%% legend.type = c("empty_m", "empty_f", ...)`

### 3.2 Tipos de Símbolos de Legenda
* `sq_empty` / `empty_m`: Quadrado vazio
* `circ_empty` / `empty_f`: Círculo vazio
* `half` / `sq_half`: Quadrado metade preenchido
* `circ_half`: Círculo metade preenchido
* `solid` / `sq_solid`: Quadrado sólido
* `circ_solid`: Círculo sólido
* `suspect` / `circ_suspect` / `sq_suspect`: Símbolo com '?'
* `proband`: Símbolo com seta 45° e 'P'
* `deceased` / `dec`: Símbolo com linha diagonal cortando

### 3.3 Atalho Auto-Legend
Quando ativado via `auto_legend` ou flag `-l` / `--legend`:
* Se o usuário não forneceu uma lista manual de itens, o renderizador inspeciona a árvore genealógica e inclui automaticamente os símbolos utilizados.
