use hashbrown::HashMap;
use portuguese_verbs_conjugator::{conjugar, get_padroes_conjugacao};

fn conjugar_verbo(verbo: &str) -> HashMap<String, String> {
    let padroes = get_padroes_conjugacao();
    conjugar(verbo, &padroes)
}

#[test]
fn test_example() {
    let verbo = "amar";
    let conjugacoes = conjugar_verbo(verbo);

    let presente_indicativo_eu =
        conjugacoes.get("presente_indicativo_eu").unwrap();

    println!("Presente indicativo eu do verbo amar é {presente_indicativo_eu}");
}
