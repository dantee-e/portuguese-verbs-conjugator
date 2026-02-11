use hashbrown::HashMap;

use super::padroes_conjugacao::Padrao;

pub fn classificar_verbo(padroes_hashmap: &HashMap<String, Padrao>, verbo: &str) -> Option<Padrao> {
    let verbo = verbo.to_uppercase();

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
    use crate::{classificador_verbos::classificar_verbo, padroes_conjugacao::Padrao};
    use hashbrown::HashMap;
    use std::fs::File;

    #[test]
    fn classificar_verbo_exatamente_padrao() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        classificar_verbo(&padroes_hashmap, "ABAULAR");
    }

    #[test]
    fn classificar_verbo_nao_exatamente_padrao() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        let result = classificar_verbo(&padroes_hashmap, "amar").unwrap();
        println!("{:#?}", result);
    }
}
