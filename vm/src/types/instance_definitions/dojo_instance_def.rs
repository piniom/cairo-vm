use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub(crate) struct DojoInstanceDef {
    pub(crate) ratio: Option<u32>,
}

impl Default for DojoInstanceDef {
    fn default() -> Self {
        DojoInstanceDef { ratio: Some(8) }
    }
}

impl DojoInstanceDef {
    pub(crate) fn new(ratio: Option<u32>) -> Self {
        DojoInstanceDef { ratio }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[cfg(target_arch = "wasm32")]
//     use wasm_bindgen_test::*;

//     #[test]
//     #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
//     fn test_new() {
//         let builtin_instance = DojoInstanceDef { ratio: Some(10) };
//         assert_eq!(DojoInstanceDef::new(Some(10)), builtin_instance);
//     }

//     #[test]
//     #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
//     fn test_default() {
//         let builtin_instance = DojoInstanceDef { ratio: Some(8) };
//         assert_eq!(DojoInstanceDef::default(), builtin_instance);
//     }
// }
