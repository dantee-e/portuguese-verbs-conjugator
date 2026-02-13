#[cfg(test)]
mod tests {
    use conjugador_verbos::conjugador_verbal::conjugar;
    use conjugador_verbos::padroes_conjugacao::Padrao;
    use hashbrown::HashMap;
    use std::fs::File;

    #[test]
    fn conjugar_verbo_amar() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        let verb = "amar".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
            "arquivos_conjugacoes/verbo_amar_conjugacao.json"
        ))
        .unwrap();

        assert_eq!(result, compare);
    }

    #[test]
    fn conjugar_verbo_ser() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        let verb = "ser".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
            "arquivos_conjugacoes/verbo_ser_conjugacao.json"
        ))
        .unwrap();

        assert_eq!(result, compare);
    }

    #[test]
    fn conjugar_verbo_ir() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        let verb = "ir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
            "arquivos_conjugacoes/verbo_ir_conjugacao.json"
        ))
        .unwrap();

        assert_eq!(result, compare);
    }
}
