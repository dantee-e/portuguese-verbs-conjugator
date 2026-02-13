import httpx
import json

verbs = [
    "fazer",
    "cansar",
    "ser",
    "viver",
]

url = "https://verbe.cc/verbecc/conjugate/pt/"


results = {}

for verb in verbs:
    request = httpx.get(url + verb)
    if request.status_code != 200:
        print("Erro: status code is ", request.status_code)
        continue

    text = json.loads(request.text)

    results[verb] = text["value"]


with open("verbs_to_test.json", "w", encoding="utf-8") as f:
    json.dump(results, f, ensure_ascii=False)
