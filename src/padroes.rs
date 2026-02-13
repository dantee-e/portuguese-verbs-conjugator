use hashbrown::HashMap;

use crate::padroes_conjugacao::Padrao;

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
pub fn from_ending_infinitive(
    padroes_hashmap: &HashMap<String, Padrao>,
    terminacao: &str,
) -> Option<Padrao> {
    match terminacao.to_uppercase().as_str() {
        "AR" => Some(ar(padroes_hashmap)),
        "ER" => Some(er(padroes_hashmap)),
        "IR" => Some(ir(padroes_hashmap)),
        _ => {
            eprintln!(
                "Invalid Infinitive ending pattern provided to from_ending function: {terminacao}"
            );
            None
        }
    }
}

pub fn from_ending(padroes_hashmap: &HashMap<String, Padrao>, padrao_str: &str) -> Option<Padrao> {
    padroes_hashmap.get(&padrao_str.to_uppercase()).cloned()
}
