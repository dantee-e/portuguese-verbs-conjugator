mod classificador_verbal;
pub mod conjugador_verbal;
pub(crate) mod helpers;

pub use classificador_verbal::classificar_verbo;
pub use conjugador_verbal::conjugar;
pub use helpers::{Padrao, get_padroes_conjugacao};
