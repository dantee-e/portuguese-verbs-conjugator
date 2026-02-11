use super::padroes_conjugacao::Padrao;

macro_rules! tentar_tempo_verbal {
    (
    $conjugacoes_vec:ident,
    $verb:ident,
    $padrao:ident,
    $padrao_infinitivo:ident,
    ( $( $tempo_verbal:ident ),+ )
    ) => {
        $(
            let terminacao = if let Some(terminacao) =
                &$padrao.terminacoes.$tempo_verbal
            {
                terminacao
            } else {
                &$padrao_infinitivo
                    .terminacoes
                    .presente_indicativo_eu
                    .clone()
                    .unwrap()
            };

            let until = $verb.len() - terminacao.remover_chars as usize;
            let root = &$verb[0..until];
            let conjugated_verb = format!("{root}{}", terminacao.terminacao);
            $conjugacoes_vec.push((stringify!($tempo_verbal).to_string(), conjugated_verb));
        )+

        // Participios (casos especiais)

        // Participio Regular
        if let Some(terminacao) = &$padrao.terminacoes.participio_regular {
            let until = $verb.len() - terminacao.remover_chars as usize;
            let root = &$verb[0..until];
            let conjugated_verb = format!("{root}{}", terminacao.terminacao);
            $conjugacoes_vec.push(("participio_regular".to_string(), conjugated_verb));
        }

        // Participio Irregular
        if let Some(terminacao) = &$padrao.terminacoes.participio_irregular {
            let until = $verb.len() - terminacao.remover_chars as usize;
            let root = &$verb[0..until];
            let conjugated_verb = format!("{root}{}", terminacao.terminacao);
            $conjugacoes_vec.push(("participio_irregular".to_string(), conjugated_verb));
        }


    };
}

pub fn conjugar(
    verb: String,
    padrao: &Padrao,
    padrao_infinitivo: &Padrao,
) -> Vec<(String, String)> {
    let mut conjugacoes: Vec<(String, String)> = Vec::new();

    tentar_tempo_verbal!(
        conjugacoes,
        verb,
        padrao,
        padrao_infinitivo,
        (
            presente_indicativo_eu,
            presente_indicativo_tu,
            presente_indicativo_ele,
            presente_indicativo_nos,
            presente_indicativo_vos,
            presente_indicativo_eles,
            preterito_perfeito_eu,
            preterito_perfeito_tu,
            preterito_perfeito_ele,
            preterito_perfeito_nos,
            preterito_perfeito_vos,
            preterito_perfeito_eles,
            preterito_imperfeito_eu,
            preterito_imperfeito_tu,
            preterito_imperfeito_ele,
            preterito_imperfeito_nos,
            preterito_imperfeito_vos,
            preterito_imperfeito_eles,
            preterito_maisqueperfeito_eu,
            preterito_maisqueperfeito_tu,
            preterito_maisqueperfeito_ele,
            preterito_maisqueperfeito_nos,
            preterito_maisqueperfeito_vos,
            preterito_maisqueperfeito_eles,
            futuro_presente_eu,
            futuro_presente_tu,
            futuro_presente_ele,
            futuro_presente_nos,
            futuro_presente_vos,
            futuro_presente_eles,
            futuro_preterito_eu,
            futuro_preterito_tu,
            futuro_preterito_ele,
            futuro_preterito_nos,
            futuro_preterito_vos,
            futuro_preterito_eles,
            presente_subjuntivo_eu,
            presente_subjuntivo_tu,
            presente_subjuntivo_ele,
            presente_subjuntivo_nos,
            presente_subjuntivo_vos,
            presente_subjuntivo_eles,
            preterito_subjuntivo_eu,
            preterito_subjuntivo_tu,
            preterito_subjuntivo_ele,
            preterito_subjuntivo_nos,
            preterito_subjuntivo_vos,
            preterito_subjuntivo_eles,
            futuro_subjuntivo_eu,
            futuro_subjuntivo_tu,
            futuro_subjuntivo_ele,
            futuro_subjuntivo_nos,
            futuro_subjuntivo_vos,
            futuro_subjuntivo_eles,
            infinitivo_pessoal_eu,
            infinitivo_pessoal_tu,
            infinitivo_pessoal_ele,
            infinitivo_pessoal_nos,
            infinitivo_pessoal_vos,
            infinitivo_pessoal_eles,
            imperativo_tu,
            imperativo_vos,
            gerundio
        )
    );
    conjugacoes
}

mod tests {
    use std::fs::File;

    use hashbrown::HashMap;

    use crate::classificador_verbos::classificar_verbo;
    use crate::conjugador_verbal::conjugar;
    use crate::padroes_conjugacao::Padrao;
    use crate::padroes_infinitivo::from_ending;

    #[test]
    fn conjugar_verbo_amar() {
        let padroes_hashmap: HashMap<String, Padrao> = serde_json::from_reader(
            File::open("src/padroes_conjugacao.json")
                .expect("No padroes_conjugacao.json file found"),
        )
        .expect("padroes_conjugacao.json is not valid JSON");

        let verb = "amar".to_string();
        let terminacao_infinitivo = &verb[&verb.len() - 2..];
        let padrao = classificar_verbo(&padroes_hashmap, &verb).unwrap();
        let padrao_infinitivo = from_ending(&padroes_hashmap, terminacao_infinitivo).unwrap();
        let result = conjugar(verb, &padrao, &padrao_infinitivo);

        for (tempo_verbal, conjugacao) in result {
            println!("{tempo_verbal}: {conjugacao}");
        }
    }
}
