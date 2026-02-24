use hashbrown::HashMap;
use once_cell::sync::Lazy;

pub(crate) static IRREGULAR_VERBS_EQUIVALENCE_MAP: Lazy<HashMap<String, String>> =
    Lazy::new(|| serde_json::from_str(include_str!("verbos_irregulares.json")).unwrap());
