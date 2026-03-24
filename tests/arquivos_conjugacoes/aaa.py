import json

with open("varios_verbos_teste.json", "r") as f:
    varios_verbos = json.load(f)

for verbo, data in varios_verbos.items():
    with open(f"verbo_{verbo}_conjugacao.json", "w") as f:
        json.dump(data, f, ensure_ascii=False)
