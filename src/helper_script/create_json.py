import pandas as pd
import json
from pathlib import Path


# Mapeamento de posições (colunas 1-64) para tempos/pessoas verbais
TEMPOS_PESSOAS = {
    1: "presente_indicativo_eu",
    2: "presente_indicativo_tu",
    3: "presente_indicativo_ele",
    4: "presente_indicativo_nos",
    5: "presente_indicativo_vos",
    6: "presente_indicativo_eles",
    7: "preterito_perfeito_eu",
    8: "preterito_perfeito_tu",
    9: "preterito_perfeito_ele",
    10: "preterito_perfeito_nos",
    11: "preterito_perfeito_vos",
    12: "preterito_perfeito_eles",
    13: "preterito_imperfeito_eu",
    14: "preterito_imperfeito_tu",
    15: "preterito_imperfeito_ele",
    16: "preterito_imperfeito_nos",
    17: "preterito_imperfeito_vos",
    18: "preterito_imperfeito_eles",
    19: "preterito_maisqueperfeito_eu",
    20: "preterito_maisqueperfeito_tu",
    21: "preterito_maisqueperfeito_ele",
    22: "preterito_maisqueperfeito_nos",
    23: "preterito_maisqueperfeito_vos",
    24: "preterito_maisqueperfeito_eles",
    25: "futuro_presente_eu",
    26: "futuro_presente_tu",
    27: "futuro_presente_ele",
    28: "futuro_presente_nos",
    29: "futuro_presente_vos",
    30: "futuro_presente_eles",
    31: "futuro_preterito_eu",
    32: "futuro_preterito_tu",
    33: "futuro_preterito_ele",
    34: "futuro_preterito_nos",
    35: "futuro_preterito_vos",
    36: "futuro_preterito_eles",
    37: "presente_subjuntivo_eu",
    38: "presente_subjuntivo_tu",
    39: "presente_subjuntivo_ele",
    40: "presente_subjuntivo_nos",
    41: "presente_subjuntivo_vos",
    42: "presente_subjuntivo_eles",
    43: "preterito_subjuntivo_eu",
    44: "preterito_subjuntivo_tu",
    45: "preterito_subjuntivo_ele",
    46: "preterito_subjuntivo_nos",
    47: "preterito_subjuntivo_vos",
    48: "preterito_subjuntivo_eles",
    49: "futuro_subjuntivo_eu",
    50: "futuro_subjuntivo_tu",
    51: "futuro_subjuntivo_ele",
    52: "futuro_subjuntivo_nos",
    53: "futuro_subjuntivo_vos",
    54: "futuro_subjuntivo_eles",
    55: "infinitivo_pessoal_eu",
    56: "infinitivo_pessoal_tu",
    57: "infinitivo_pessoal_ele",
    58: "infinitivo_pessoal_nos",
    59: "infinitivo_pessoal_vos",
    60: "infinitivo_pessoal_eles",
    61: "gerundio",
    63: "imperativo_tu",
    64: "imperativo_vos",
}

padroes_variaveis = [
    "Abundante",
    "Defectivo 1",
    "Defectivo 2",
    "Defectivo 3",
    "defectivo 3",
    "Defectivo 4",
    "Defectivo 5",
    "Defectivo 6",
    "E-X-IR",
    "Í-X-AR",
    "O-X-IR",
    "Ú-X-AR",
    "GAR",
    "GER",
    "'´-GUAR",
    "QUERER",
    "POR",
]


def extrair_padroes(arquivo_excel):
    df_padroes = pd.read_excel(arquivo_excel, sheet_name="Padrões", header=0)

    # Estrutura do JSON
    resultado = {}

    for _, row in df_padroes.iterrows():
        verbo_modelo: str = row["Verbo"]
        nome_padrao = row["Padrão"]

        # Pula linhas invalidas, nao sei se tem alguma
        # if pd.isna(verbo_modelo) or pd.isna(nome_padrao):
        #     continue

        # Ignora linhas de controle
        if verbo_modelo in ["Total", "xxxxx"] or nome_padrao in ["nan", "xxx"]:
            continue

        # Criar identificador único para o padrão
        # Se o verbo modelo for diferente do nome do padrão, criar ID composto
        if " regular" in nome_padrao:
            padrao_id = nome_padrao.replace(" regular", "")
        elif nome_padrao.lower() == "ir":
            padrao_id = "ir_verbo"
        elif nome_padrao.lower() == "ser":
            padrao_id = "ser_verbo"
        elif (
            " ê" in nome_padrao.lower()
            or " é" in nome_padrao.lower()
            or " ó" in nome_padrao.lower()
            or " ô" in nome_padrao.lower()
        ):
            print("entered the elif")
            padrao_id = (
                nome_padrao.replace(" é", "")
                .replace(" ê", "")
                .replace(" ó", "")
                .replace(" ô", "")
            )

        elif nome_padrao in (padroes_variaveis):
            padrao_id = verbo_modelo.upper()
        else:
            padrao_id = nome_padrao

        terminacoes = {}

        for pos in range(1, 65):
            # Pular participio, caso especial
            if pos == 62:
                continue

            col_numero_nome = f"Coluna{pos * 2 + 1}"
            col_terminacao = str(pos)

            # Verificar se as colunas existem
            if col_terminacao not in df_padroes.columns:
                continue

            # Pegar valores
            terminacao = row[col_terminacao]

            # Pegar número de caracteres a remover (da coluna anterior)
            num_chars = None
            if col_numero_nome in df_padroes.columns:
                num_chars = row[col_numero_nome]

            # Se há terminação válida
            if pd.notna(terminacao):
                # Converter para string e limpar
                terminacao = str(terminacao).strip()

                # Ignorar marcadores especiais que não são terminações
                if terminacao in ["I", "A", "R", ""]:
                    continue

                # Pegar nome do tempo/pessoa
                tempo_pessoa = TEMPOS_PESSOAS.get(pos, f"posicao_{pos}")

                # Adicionar ao dicionário
                terminacoes[tempo_pessoa] = {
                    "remover_chars": int(num_chars) if pd.notna(num_chars) else 2,
                    "terminacao": terminacao,
                }

        # Participios

        col_terminacao = "62"
        participio_regular: bool = bool(
            row["Coluna125"] if pd.notna(row["Coluna125"]) else False
        )
        participio_irregular: int = int(
            row["Coluna126"] if pd.notna(row["Coluna126"]) else 0
        )
        terminacao = row[col_terminacao]

        # Pegar valores
        terminacao_regular = None
        match verbo_modelo[-2]:
            case "a":
                terminacao_regular = "ado"
            case "i":
                terminacao_regular = "ido"
            case "e":
                terminacao_regular = "ido"
            # gambiarra para o por
            case "o":
                terminacao_regular = "osto"

        if participio_regular:
            terminacoes["participio_regular"] = {
                "remover_chars": 2,
                "terminacao": terminacao_regular,
            }
        if participio_irregular:
            terminacoes["participio_irregular"] = {
                "remover_chars": participio_irregular,
                "terminacao": terminacao,
            }

        if padrao_id in (["ER", "IR"]):
            terminacoes["participio_regular"] = {
                "remover_chars": 2,
                "terminacao": "ido",
            }
        elif padrao_id == "AR":
            terminacoes["participio_regular"] = {
                "remover_chars": 2,
                "terminacao": "ado",
            }
        elif padrao_id == "PÔR":  # ← ADICIONAR
            terminacoes["participio_regular"] = {
                "remover_chars": 2,
                "terminacao": "osto",
            }
        elif participio_regular and terminacao_regular:  # ← ADICIONAR condição
            # Para outros casos que tenham participio_regular marcado
            terminacoes["participio_regular"] = {
                "remover_chars": 2,
                "terminacao": terminacao_regular,
            }

        # observacoes = row.get("Observações", None)
        # if pd.notna(observacoes):
        #     observacoes = str(observacoes).strip()
        # else:
        #     observacoes = None

        # Adicionar padrão ao resultado
        resultado[padrao_id] = {
            "nome": padrao_id,
            "verbo_modelo": verbo_modelo,
            "terminacoes": terminacoes,
            # "observacoes": observacoes,
        }

    return resultado


def salvar_json(dados, arquivo_saida, indent=2):
    with open(arquivo_saida, "w", encoding="utf-8") as f:
        json.dump(dados, f, ensure_ascii=False, indent=indent)

    tamanho = Path(arquivo_saida).stat().st_size
    print(f"\n✓ Arquivo salvo: {arquivo_saida}")
    print(f"  Tamanho: {tamanho:,} bytes ({tamanho / 1024:.1f} KB)")


def main():
    arquivo_entrada = "conjugador-de-verbos-1-2-2-excelled.xlsx"

    # Arquivos de saída
    arquivo_completo = "padroes_conjugacao.json"

    try:
        dados = extrair_padroes(arquivo_entrada)
        salvar_json(dados, arquivo_completo)

    except Exception as e:
        print(f"ERROR: {e}")


def create_rust_terminacoes():
    print("pub struct Terminacoes {")
    for tempo_verbal in TEMPOS_PESSOAS:
        key = TEMPOS_PESSOAS[tempo_verbal]
        print()
        print(f'#[serde(rename = "{key}")]')
        print(f"pub {key}: Option<TerminacaoRule>,")
    print("}")


if __name__ == "__main__":
    main()
