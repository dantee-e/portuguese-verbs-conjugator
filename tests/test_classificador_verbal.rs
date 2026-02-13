#[cfg(test)]
mod tests {
    use conjugador_verbos::padroes::from_ending;
    use conjugador_verbos::{classificador_verbal::classificar_verbo, padroes_conjugacao::Padrao};
    use hashbrown::HashMap;
    use std::fs::File;

    #[test]
    fn classificar_verbo_regular() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        let result = classificar_verbo(&padroes_hashmap, "amar").unwrap();
        println!("{:#?}", result);
    }

    #[test]
    fn classificar_verbo_ser() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        let result = classificar_verbo(&padroes_hashmap, "ser").unwrap();
        let expected =
            from_ending(&padroes_hashmap, "ser_verbo").expect("did not find pattern ser_verbo");
        assert_eq!(result, expected);
    }
}
