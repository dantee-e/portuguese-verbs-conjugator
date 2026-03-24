# PT 
Esse é um conjugador de verbos em português, elaborado baseado na planilha do Excel criada por Radamés Manosso. O link para a planilha pode ser encontrado [aqui](https://radames.manosso.nom.br/bitabit/planilhas/conjugador-de-verbos-em-excel/).

O projeto foi iniciado para auxiliar a implementação do [Harper](https://writewithharper.com/docs/about) para a língua portuguesa, atualmente [nesse PR](https://github.com/Automattic/harper/pull/2150)
## Utilização
Para usar a crate, as funções mais importantes são as de obter o HashMap dos padrões de conjugações e a função de conjugação em si. Ambas podem ser importadas usando

```rust
use portuguese_verbs_conjugator::{conjugar, get_padroes_conjugacao};
```

Depois de importar as funções, armazene o HashMap em uma variável (vale citar que esse é um hashbrown::HashMap):

```rust
let padroes = get_padroes_conjugacao();
```

E por fim, conjugue o verbo utilizando a função conjugar:

```rust
let conjugacoes = conjugar(verbo, &padroes)
```

A função conjugar retorna um HashMap<String, String>. Para acessar uma conjugacao especifica, basta tentar acessá-la da seguinte forma:

```rust
let presente_indicativo_eu =
    conjugacoes.get("presente_indicativo_eu").except("Conjugação não encontrada");
println!("{presente_indicativo_eu}");
```

A lista de conjugações é:

```
presente_indicativo_eu,
presente_indicativo_tu,
presente_indicativo_ele,
presente_indicativo_nos,
presente_indicativo_vos,
presente_indicativo_eles,
preterito_perfeito_eu,
preterito_perfeito_tu,
preterito_perfeito_ele,
preterito_perfeito_nos,
preterito_perfeito_vos,
preterito_perfeito_eles,
preterito_imperfeito_eu,
preterito_imperfeito_tu,
preterito_imperfeito_ele,
preterito_imperfeito_nos,
preterito_imperfeito_vos,
preterito_imperfeito_eles,
preterito_maisqueperfeito_eu,
preterito_maisqueperfeito_tu,
preterito_maisqueperfeito_ele,
preterito_maisqueperfeito_nos,
preterito_maisqueperfeito_vos,
preterito_maisqueperfeito_eles,
futuro_presente_eu,
futuro_presente_tu,
futuro_presente_ele,
futuro_presente_nos,
futuro_presente_vos,
futuro_presente_eles,
futuro_preterito_eu,
futuro_preterito_tu,
futuro_preterito_ele,
futuro_preterito_nos,
futuro_preterito_vos,
futuro_preterito_eles,
presente_subjuntivo_eu,
presente_subjuntivo_tu,
presente_subjuntivo_ele,
presente_subjuntivo_nos,
presente_subjuntivo_vos,
presente_subjuntivo_eles,
preterito_subjuntivo_eu,
preterito_subjuntivo_tu,
preterito_subjuntivo_ele,
preterito_subjuntivo_nos,
preterito_subjuntivo_vos,
preterito_subjuntivo_eles,
futuro_subjuntivo_eu,
futuro_subjuntivo_tu,
futuro_subjuntivo_ele,
futuro_subjuntivo_nos,
futuro_subjuntivo_vos,
futuro_subjuntivo_eles,
infinitivo_pessoal_eu,
infinitivo_pessoal_tu,
infinitivo_pessoal_ele,
infinitivo_pessoal_nos,
infinitivo_pessoal_vos,
infinitivo_pessoal_eles,
imperativo_tu,
imperativo_vos,
gerundio,
participio_regular,
participio_irregular
```

Vale chamar atenção para o fato de que muitas vezes não estão disponíveis as conjugações de ambos os particípios.

# EN
This is a verb conjugator for the Portuguese language, made based on the Excel spreadsheet by Radamés Manosso. The spreadsheet can be found [here](https://radames.manosso.nom.br/bitabit/planilhas/conjugador-de-verbos-em-excel/).

The project was created to help porting the [Harper Grammar Checker](https://writewithharper.com/docs/about) to Portuguese. You can find the PR [here](https://github.com/Automattic/harper/pull/2150) 
