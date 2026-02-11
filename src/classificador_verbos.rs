use hashbrown::HashMap;
use std::fs::File;

use super::padroes_conjugacao::Padrao;

use serde_json;

fn classificar_verbo(verbo: String) -> Option<Padrao> {
    let verbo = verbo.to_uppercase();
    let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
        File::open("src/padroes_conjugacao.json").expect("No padroes_conjugacao.json file found"),
    )
    .expect("padroes_conjugacao.json is not valid JSON");

    // let all_patterns = padroes_hashmap.keys();

    if let Some(padrao) = padroes_hashmap.get(&verbo) {
        return Some(padrao.clone());
    }

    let number_of_letters_in_verb = verbo.len();

    for i in 0..=number_of_letters_in_verb {
        println!("Comparing {}", &verbo[i..number_of_letters_in_verb]);
        if let Some(padrao) = padroes_hashmap.get(&verbo[i..number_of_letters_in_verb]) {
            return Some(padrao.clone());
        }
    }
    None
}

mod tests {
    use crate::classificador_verbos::classificar_verbo;

    #[test]
    fn verb_that_is_pattern() {
        classificar_verbo("ABAULAR".to_string());
    }

    #[test]
    fn verb_that_is_not_a_pattern() {
        let result = classificar_verbo("amar".to_string()).unwrap();
        println!("{:#?}", result);
    }
}
