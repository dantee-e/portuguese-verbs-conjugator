use hashbrown::HashMap;

use super::padroes_conjugacao::Padrao;

pub fn classificar_verbo(padroes_hashmap: &HashMap<String, Padrao>, verbo: &str) -> Option<Padrao> {
    let verbo = verbo.to_uppercase();

    // verbos excecoes: ir e ser
    let lookup = if verbo == "IR" || verbo == "SER" {
        format!("{verbo}_VERBO")
    } else {
        verbo.clone()
    };

    if let Some(padrao) = padroes_hashmap.get(&lookup) {
        return Some(padrao.clone());
    }

    let number_of_letters_in_verb = verbo.len();

    for i in 0..=number_of_letters_in_verb {
        if let Some(padrao) = padroes_hashmap.get(&verbo[i..number_of_letters_in_verb]) {
            return Some(padrao.clone());
        }
    }
    None
}
