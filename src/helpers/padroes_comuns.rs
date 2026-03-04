use hashbrown::HashMap;

use super::Padrao;

pub fn ar(padroes_hashmap: &HashMap<String, Padrao>) -> Padrao {
    if let Some(padrao) = padroes_hashmap.get("AR") {
        padrao.clone()
    } else {
        panic!("Infinitive pattern not found in padroes_hashmap");
    }
}
pub fn er(padroes_hashmap: &HashMap<String, Padrao>) -> Padrao {
    if let Some(padrao) = padroes_hashmap.get("ER") {
        padrao.clone()
    } else {
        panic!("Infinitive pattern not found in padroes_hashmap");
    }
}
pub fn ir(padroes_hashmap: &HashMap<String, Padrao>) -> Padrao {
    if let Some(padrao) = padroes_hashmap.get("IR") {
        padrao.clone()
    } else {
        panic!("Infinitive pattern not found in padroes_hashmap");
    }
}
fn por(padroes_hashmap: &HashMap<String, Padrao>) -> Padrao {
    if let Some(padrao) = padroes_hashmap.get("PÔR") {
        padrao.clone()
    } else {
        panic!("Infinitive pattern not found in padroes_hashmap");
    }
}

#[allow(dead_code)]
pub fn get_infinitive_pattern(
    padroes_hashmap: &HashMap<String, Padrao>,
    verb: &str,
) -> Option<Padrao> {
    // let ending = verb
    //     .to_uppercase()
    //     .chars()
    //     .rev()
    //     .take(2)
    //     .collect::<String>();
    //
    // println!("Trying verb {verb} with ending {ending}");

    match verb
        .to_uppercase()
        .chars()
        .rev()
        .take(2)
        .collect::<String>()
        .as_str()
    {
        "RA" => Some(ar(padroes_hashmap)),
        "RE" => Some(er(padroes_hashmap)),
        "RI" => Some(ir(padroes_hashmap)),
        "RÔ" => Some(por(padroes_hashmap)),
        x => {
            eprintln!(
                "Invalid Infinitive ending pattern ({x}) provided to from_ending function: {verb}"
            );
            None
        }
    }
}

#[allow(dead_code)]
pub fn from_ending(
    padroes_hashmap: &HashMap<String, Padrao>,
    padrao_str: &str,
) -> Option<Padrao> {
    padroes_hashmap.get(&padrao_str.to_uppercase()).cloned()
}

#[cfg(test)]
mod tests {
    use crate::get_padroes_conjugacao;

    use super::*;

    #[test]
    fn test_get_infinitive_pattern() {
        let padroes_hashmap = get_padroes_conjugacao();
        let ar_local = (
            get_infinitive_pattern(&padroes_hashmap, "amar").unwrap(),
            ar(&padroes_hashmap),
        );
        let er_local = (
            get_infinitive_pattern(&padroes_hashmap, "ser").unwrap(),
            er(&padroes_hashmap),
        );
        let ir_local = (
            get_infinitive_pattern(&padroes_hashmap, "sair").unwrap(),
            ir(&padroes_hashmap),
        );
        let por_local = (
            get_infinitive_pattern(&padroes_hashmap, "pôr").unwrap(),
            por(&padroes_hashmap),
        );

        assert_eq!(ar_local.0, ar_local.1);
        assert_eq!(er_local.0, er_local.1);
        assert_eq!(ir_local.0, ir_local.1);
        assert_eq!(por_local.0, por_local.1);
    }
}
