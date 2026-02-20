use std::fs::File;

use hashbrown::HashMap;
use serde;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminacaoRule {
    pub remover_chars: u8,
    pub terminacao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Padrao {
    pub nome: String,

    pub verbo_modelo: String,

    pub terminacoes: Terminacoes,

    pub observacoes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Terminacoes {
    #[serde(rename = "presente_indicativo_eu")]
    pub presente_indicativo_eu: Option<TerminacaoRule>,

    #[serde(rename = "presente_indicativo_tu")]
    pub presente_indicativo_tu: Option<TerminacaoRule>,

    #[serde(rename = "presente_indicativo_ele")]
    pub presente_indicativo_ele: Option<TerminacaoRule>,

    #[serde(rename = "presente_indicativo_nos")]
    pub presente_indicativo_nos: Option<TerminacaoRule>,

    #[serde(rename = "presente_indicativo_vos")]
    pub presente_indicativo_vos: Option<TerminacaoRule>,

    #[serde(rename = "presente_indicativo_eles")]
    pub presente_indicativo_eles: Option<TerminacaoRule>,

    #[serde(rename = "preterito_perfeito_eu")]
    pub preterito_perfeito_eu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_perfeito_tu")]
    pub preterito_perfeito_tu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_perfeito_ele")]
    pub preterito_perfeito_ele: Option<TerminacaoRule>,

    #[serde(rename = "preterito_perfeito_nos")]
    pub preterito_perfeito_nos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_perfeito_vos")]
    pub preterito_perfeito_vos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_perfeito_eles")]
    pub preterito_perfeito_eles: Option<TerminacaoRule>,

    #[serde(rename = "preterito_imperfeito_eu")]
    pub preterito_imperfeito_eu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_imperfeito_tu")]
    pub preterito_imperfeito_tu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_imperfeito_ele")]
    pub preterito_imperfeito_ele: Option<TerminacaoRule>,

    #[serde(rename = "preterito_imperfeito_nos")]
    pub preterito_imperfeito_nos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_imperfeito_vos")]
    pub preterito_imperfeito_vos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_imperfeito_eles")]
    pub preterito_imperfeito_eles: Option<TerminacaoRule>,

    #[serde(rename = "preterito_maisqueperfeito_eu")]
    pub preterito_maisqueperfeito_eu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_maisqueperfeito_tu")]
    pub preterito_maisqueperfeito_tu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_maisqueperfeito_ele")]
    pub preterito_maisqueperfeito_ele: Option<TerminacaoRule>,

    #[serde(rename = "preterito_maisqueperfeito_nos")]
    pub preterito_maisqueperfeito_nos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_maisqueperfeito_vos")]
    pub preterito_maisqueperfeito_vos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_maisqueperfeito_eles")]
    pub preterito_maisqueperfeito_eles: Option<TerminacaoRule>,

    #[serde(rename = "futuro_presente_eu")]
    pub futuro_presente_eu: Option<TerminacaoRule>,

    #[serde(rename = "futuro_presente_tu")]
    pub futuro_presente_tu: Option<TerminacaoRule>,

    #[serde(rename = "futuro_presente_ele")]
    pub futuro_presente_ele: Option<TerminacaoRule>,

    #[serde(rename = "futuro_presente_nos")]
    pub futuro_presente_nos: Option<TerminacaoRule>,

    #[serde(rename = "futuro_presente_vos")]
    pub futuro_presente_vos: Option<TerminacaoRule>,

    #[serde(rename = "futuro_presente_eles")]
    pub futuro_presente_eles: Option<TerminacaoRule>,

    #[serde(rename = "futuro_preterito_eu")]
    pub futuro_preterito_eu: Option<TerminacaoRule>,

    #[serde(rename = "futuro_preterito_tu")]
    pub futuro_preterito_tu: Option<TerminacaoRule>,

    #[serde(rename = "futuro_preterito_ele")]
    pub futuro_preterito_ele: Option<TerminacaoRule>,

    #[serde(rename = "futuro_preterito_nos")]
    pub futuro_preterito_nos: Option<TerminacaoRule>,

    #[serde(rename = "futuro_preterito_vos")]
    pub futuro_preterito_vos: Option<TerminacaoRule>,

    #[serde(rename = "futuro_preterito_eles")]
    pub futuro_preterito_eles: Option<TerminacaoRule>,

    #[serde(rename = "presente_subjuntivo_eu")]
    pub presente_subjuntivo_eu: Option<TerminacaoRule>,

    #[serde(rename = "presente_subjuntivo_tu")]
    pub presente_subjuntivo_tu: Option<TerminacaoRule>,

    #[serde(rename = "presente_subjuntivo_ele")]
    pub presente_subjuntivo_ele: Option<TerminacaoRule>,

    #[serde(rename = "presente_subjuntivo_nos")]
    pub presente_subjuntivo_nos: Option<TerminacaoRule>,

    #[serde(rename = "presente_subjuntivo_vos")]
    pub presente_subjuntivo_vos: Option<TerminacaoRule>,

    #[serde(rename = "presente_subjuntivo_eles")]
    pub presente_subjuntivo_eles: Option<TerminacaoRule>,

    #[serde(rename = "preterito_subjuntivo_eu")]
    pub preterito_subjuntivo_eu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_subjuntivo_tu")]
    pub preterito_subjuntivo_tu: Option<TerminacaoRule>,

    #[serde(rename = "preterito_subjuntivo_ele")]
    pub preterito_subjuntivo_ele: Option<TerminacaoRule>,

    #[serde(rename = "preterito_subjuntivo_nos")]
    pub preterito_subjuntivo_nos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_subjuntivo_vos")]
    pub preterito_subjuntivo_vos: Option<TerminacaoRule>,

    #[serde(rename = "preterito_subjuntivo_eles")]
    pub preterito_subjuntivo_eles: Option<TerminacaoRule>,

    #[serde(rename = "futuro_subjuntivo_eu")]
    pub futuro_subjuntivo_eu: Option<TerminacaoRule>,

    #[serde(rename = "futuro_subjuntivo_tu")]
    pub futuro_subjuntivo_tu: Option<TerminacaoRule>,

    #[serde(rename = "futuro_subjuntivo_ele")]
    pub futuro_subjuntivo_ele: Option<TerminacaoRule>,

    #[serde(rename = "futuro_subjuntivo_nos")]
    pub futuro_subjuntivo_nos: Option<TerminacaoRule>,

    #[serde(rename = "futuro_subjuntivo_vos")]
    pub futuro_subjuntivo_vos: Option<TerminacaoRule>,

    #[serde(rename = "futuro_subjuntivo_eles")]
    pub futuro_subjuntivo_eles: Option<TerminacaoRule>,

    #[serde(rename = "infinitivo_pessoal_eu")]
    pub infinitivo_pessoal_eu: Option<TerminacaoRule>,

    #[serde(rename = "infinitivo_pessoal_tu")]
    pub infinitivo_pessoal_tu: Option<TerminacaoRule>,

    #[serde(rename = "infinitivo_pessoal_ele")]
    pub infinitivo_pessoal_ele: Option<TerminacaoRule>,

    #[serde(rename = "infinitivo_pessoal_nos")]
    pub infinitivo_pessoal_nos: Option<TerminacaoRule>,

    #[serde(rename = "infinitivo_pessoal_vos")]
    pub infinitivo_pessoal_vos: Option<TerminacaoRule>,

    #[serde(rename = "infinitivo_pessoal_eles")]
    pub infinitivo_pessoal_eles: Option<TerminacaoRule>,

    #[serde(rename = "imperativo_tu")]
    pub imperativo_tu: Option<TerminacaoRule>,

    #[serde(rename = "participio_irregular")]
    pub participio_irregular: Option<TerminacaoRule>,

    #[serde(rename = "participio_regular")]
    pub participio_regular: Option<TerminacaoRule>,

    #[serde(rename = "imperativo_vos")]
    pub imperativo_vos: Option<TerminacaoRule>,

    #[serde(rename = "gerundio")]
    pub gerundio: Option<TerminacaoRule>,
}

pub fn get_padroes_conjugacao() -> HashMap<String, Padrao> {
    serde_json::from_reader(
        File::open("src/padroes_conjugacao.json").expect("No padroes_conjugacao.json file found"),
    )
    .expect("padroes_conjugacao.json is not valid JSON")
}
