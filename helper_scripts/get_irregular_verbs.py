import pandas as pd
import json

df = pd.read_excel(
    "conjugador-de-verbos-desprotegido.xlsx", sheet_name="Verbos irregulares"
)

# padroes_variaveis = [
#     "Abundante",
#     "Defectivo 1",
#     "Defectivo 2",
#     "Defectivo 3",
#     "defectivo 3",
#     "Defectivo 4",
#     "Defectivo 5",
#     "Defectivo 6",
#     "E-X-IR",
#     "Í-X-AR",
#     "O-X-IR",
#     "Ú-X-AR",
#     "GAR",
#     "GER",
#     "'´-GUAR",
#     "QUERER",
#     "POR",
# ]


def upper(x):
    return x.upper()


df = df.drop("Padrão", axis=1)
df = df.rename(columns={"Verbo": "verb", "Similar a": "similar_to"})
df["verb"] = df["verb"].apply(upper)
df["similar_to"] = df["similar_to"].apply(upper)

verb_dict = dict(zip(df["verb"], df["similar_to"]))

with open("verbos_irregulares.json", "w", encoding="utf-8") as f:
    json.dump(verb_dict, f, ensure_ascii=False, indent=2)
