#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use portuguese_verbs_conjugator::conjugador_verbal::conjugar;
    use portuguese_verbs_conjugator::padroes_conjugacao::{Padrao, get_padroes_conjugacao};

    fn compare_conjugacoes(
        resultado: &HashMap<String, String>,
        gabarito: &HashMap<String, String>,
    ) {
        for (tempo, verbo_conjugado) in resultado {
            if let Some(verbo_conjugado_gabarito) = gabarito.get(tempo) {
                assert_eq!(verbo_conjugado, verbo_conjugado_gabarito);
            }
        }
    }
    #[test]
    fn conjugar_verbo_amar() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();

        let verb = "amar".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
            "arquivos_conjugacoes/verbo_amar_conjugacao.json"
        ))
        .unwrap();

        compare_conjugacoes(&result, &compare);
    }

    #[test]
    fn conjugar_verbo_ser() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();

        let verb = "ser".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
            "arquivos_conjugacoes/verbo_ser_conjugacao.json"
        ))
        .unwrap();

        compare_conjugacoes(&result, &compare);
    }

    #[test]
    fn conjugar_verbo_ir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();

        let verb = "ir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
            "arquivos_conjugacoes/verbo_ir_conjugacao.json"
        ))
        .unwrap();

        compare_conjugacoes(&result, &compare);
    }

    #[test]
    fn conjugar_verbos_arquivo_teste() {
        let padroes_hashmap = get_padroes_conjugacao();
        let compare_list: HashMap<String, HashMap<String, String>> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/varios_verbos_teste.json"),
        )
        .unwrap();

        for (verbo, conjugacao_json) in compare_list {
            let conjugacao = conjugar(&verbo, &padroes_hashmap);
            compare_conjugacoes(&conjugacao, &conjugacao_json);
        }
    }

    #[test]
    fn conjugar_verbo_fazer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();

        let verb = "fazer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
            "arquivos_conjugacoes/verbo_ir_conjugacao.json"
        ))
        .unwrap();

        compare_conjugacoes(&result, &compare);
    }
}
