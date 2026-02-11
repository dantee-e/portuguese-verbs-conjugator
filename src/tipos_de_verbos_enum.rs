use strum_macros::{Display, EnumString};

enum PrioridadeTerminacao {
    Generico,
    Unico,
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum PadroesRigidos {
    AIR,
    ANGER,
    AR,
    ARGUIR,
    AZER,
    CABER,
    CAR,
    CER,
    CERZIR,
    COAR,
    COBRIR,
    CRER,
    DAR,
    Defectivo5,
    Defectivo6,
    DENEGRIR,
    DESPROVER,
    DIZER,
    DORMIR,
    EAR,
    // ECHAREacute,
    // ECHAREcirc,
    ECHAR,
    // EJAROacute,
    // EJAROcirc,
    EJAR,
    ELHAR,
    EMBAUCAR,
    ENGOLIR,
    ENTUPIR,
    ER,
    ERGUER,
    ESTAR,
    EXTINGUIR,
    FAZER,
    FRIGIR,
    FUGIR,
    GREDIR,
    GUAR,
    HAVER,
    IAR,
    IDEAR,
    INCLUIR,
    IR,
    IR_vir,
    MOSCAR,
    OAR,
    OER,
    OIAR,
    PARAR,
    PARIR,
    PEDIR,
    PELAR,
    PERDER,
    PODER,
    POLIR,
    POR,
    PREVENIR,
    PROVER,
    REQUERER,
    RESFOLEGAR,
    REUNIR,
    RIR,
    SABER,
    SENTIR,
    SER,
    SORTIR,
    SUMIR,
    TER,
    TOSSIR,
    TRAZER,
    UIR,
    UZIR,
    VALER,
    VER,
    VIR,
    X_TER,
    NotAVerb,
}
/*
* Padroes nao rigidos sao:
* Abundante
* Defectivo 1
* Defectivo 2
* Defectivo 3
* Defectivo 4
* E-X-IR
* Í-X-AR
* O-X-IR
* Ú-X-AR
* GAR
* GER
* QUERER
*/
#[cfg(test)]
mod tests {
    use super::PadroesRigidos;

    #[test]
    fn serialize() {
        let to_serialize = PadroesRigidos::AIR;
        let str = to_serialize.to_string();
        println!("{str}");
        str.parse::<PadroesRigidos>().unwrap();
    }
}
