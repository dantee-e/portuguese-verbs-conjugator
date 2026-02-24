mod all_irregular_verbs;

use crate::helpers::padroes_comuns::get_infinitive_pattern;

use super::helpers::Padrao;
use all_irregular_verbs::IRREGULAR_VERBS_EQUIVALENCE_MAP;
use hashbrown::HashMap;

pub fn classificar_verbo(
    padroes_hashmap: &HashMap<String, Padrao>,
    verbo_original: &str,
) -> Option<Padrao> {
    let verbo = if let Some(verb_with_equivalent_pattern) =
        IRREGULAR_VERBS_EQUIVALENCE_MAP.get(&verbo_original.to_uppercase())
    {
        verb_with_equivalent_pattern.clone()
    } else {
        return get_infinitive_pattern(padroes_hashmap, verbo_original);
    };

    println!("Trying to get exact match for verb {verbo}");
    // verbos excecoes: ir, ser e ter
    let lookup = if verbo == "IR" || verbo == "SER" || verbo == "TER" {
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
            println!(
                "Padrao encontrado para verbo {verbo_original}: {}",
                padrao.nome
            );
            return Some(padrao.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::super::helpers::padroes_comuns::from_ending;
    use crate::{Padrao, classificar_verbo, get_padroes_conjugacao, helpers::padroes_comuns::ar};
    use hashbrown::HashMap;

    #[test]
    fn classificar_verbo_regular() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();

        let result = classificar_verbo(&padroes_hashmap, "amar").unwrap();
        assert_eq!(result, ar(&padroes_hashmap));
    }

    #[test]
    fn classificar_verbo_ser() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();

        let result = classificar_verbo(&padroes_hashmap, "ser").unwrap();
        let expected =
            from_ending(&padroes_hashmap, "ser_verbo").expect("did not find pattern ser_verbo");
        assert_eq!(result, expected);
    }
}
