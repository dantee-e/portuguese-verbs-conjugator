#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use portuguese_verbs_conjugator::conjugador_verbal::conjugar;
    use portuguese_verbs_conjugator::{Padrao, get_padroes_conjugacao};

    fn compare_conjugacoes(
        resultado: &HashMap<String, String>,
        gabarito: &HashMap<String, String>,
    ) {
        for (tempo, verbo_conjugado) in resultado {
            println!("Testando tempo {tempo}");
            if let Some(verbo_conjugado_gabarito) = gabarito.get(tempo) {
                assert_eq!(verbo_conjugado, verbo_conjugado_gabarito);
            }
        }
    }

    // #[ignore = "separei entre varios arquivos"]
    #[test]
    fn conjugar_verbos_arquivo_teste() {
        let padroes_hashmap = get_padroes_conjugacao();
        let compare_list: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(include_str!(
                "arquivos_conjugacoes/varios_verbos_teste.json"
            ))
            .unwrap();
        for (verbo, conjugacao_json) in compare_list {
            println!(
                "Tentando conjugar o verbo {verbo} no teste com varios verbos"
            );
            let conjugacao = conjugar(&verbo, &padroes_hashmap);
            compare_conjugacoes(&conjugacao, &conjugacao_json);
        }
    }
    #[test]
    fn conjugar_verbo_abrir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "abrir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_abrir_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_amar() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "amar".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_amar_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_andar() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "andar".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_andar_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_caber() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "caber".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_caber_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_comer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "comer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_comer_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_dar() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "dar".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_dar_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_dizer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "dizer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_dizer_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_escrever() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "escrever".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_escrever_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_falar() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "falar".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_falar_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_fazer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "fazer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_fazer_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_haver() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "haver".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_haver_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_ir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "ir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_ir_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_ouvir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "ouvir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_ouvir_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_partir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "partir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_partir_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_perder() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "perder".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_perder_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_poder() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "poder".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_poder_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_por() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "pôr".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(include_str!(
        "arquivos_conjugacoes/verbo_por_conjugacao.json"
    ))
    .expect("Error converting the file viver_verbo_conjugacao.json into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_querer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "querer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_querer_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_rir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "rir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_rir_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_saber() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "saber".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_saber_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_ser() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "ser".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_ser_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_ter() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "ter".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_ter_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_trazer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "trazer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_trazer_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_valer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "valer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_valer_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_ver() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "ver".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_ver_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_vir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "vir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_vir_conjugacao.json"),
        )
        .unwrap();
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_viver() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "viver".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_viver_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_averiguar() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "averiguar".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> =
            serde_json::from_str(include_str!(
                "arquivos_conjugacoes/verbo_averiguar_conjugacao.json"
            ))
            .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_crer() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "crer".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_crer_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_digerir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "digerir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_digerir_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_fugir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "fugir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_fugir_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_medir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "medir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_medir_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_passear() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "passear".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_passear_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_pedir() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "pedir".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_pedir_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
    #[test]
    fn conjugar_verbo_sair() {
        let padroes_hashmap: HashMap<String, Padrao> = get_padroes_conjugacao();
        let verb = "sair".to_string();
        let result = conjugar(&verb, &padroes_hashmap);
        let compare: HashMap<String, String> = serde_json::from_str(
            include_str!("arquivos_conjugacoes/verbo_sair_conjugacao.json"),
        )
        .expect("Error converting the file into a serde Value");
        compare_conjugacoes(&result, &compare);
    }
}
