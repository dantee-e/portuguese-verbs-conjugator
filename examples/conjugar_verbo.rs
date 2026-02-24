use hashbrown::HashMap;
use portuguese_verbs_conjugator::{conjugar, get_padroes_conjugacao};

fn conjugar_verbo(verbo: &str) -> HashMap<String, String> {
    let padroes = get_padroes_conjugacao();
    conjugar(verbo, &padroes)
}

fn main() {
    let verbo = "amar";
    let conjugacoes = conjugar_verbo(verbo);

    let presente_indicativo_eu = conjugacoes.get("presente_indicativo_eu").unwrap();
    println!("{presente_indicativo_eu}");
}
