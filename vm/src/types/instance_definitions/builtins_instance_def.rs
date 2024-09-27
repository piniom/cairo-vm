use super::dojo_instance_def::DojoInstanceDef;
use super::mod_instance_def::ModInstanceDef;
use super::{
    bitwise_instance_def::BitwiseInstanceDef, ec_op_instance_def::EcOpInstanceDef,
    ecdsa_instance_def::EcdsaInstanceDef, keccak_instance_def::KeccakInstanceDef,
    pedersen_instance_def::PedersenInstanceDef, poseidon_instance_def::PoseidonInstanceDef,
    range_check_instance_def::RangeCheckInstanceDef,
};

pub(crate) const BUILTIN_INSTANCES_PER_COMPONENT: u32 = 1;

use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub(crate) struct BuiltinsInstanceDef {
    pub(crate) output: bool,
    pub(crate) pedersen: Option<PedersenInstanceDef>,
    pub(crate) range_check: Option<RangeCheckInstanceDef>,
    pub(crate) ecdsa: Option<EcdsaInstanceDef>,
    pub(crate) bitwise: Option<BitwiseInstanceDef>,
    pub(crate) ec_op: Option<EcOpInstanceDef>,
    pub(crate) keccak: Option<KeccakInstanceDef>,
    pub(crate) poseidon: Option<PoseidonInstanceDef>,
    pub(crate) range_check96: Option<RangeCheckInstanceDef>,
    pub(crate) add_mod: Option<ModInstanceDef>,
    pub(crate) mul_mod: Option<ModInstanceDef>,
    pub(crate) dojo: Option<DojoInstanceDef>,
}

impl BuiltinsInstanceDef {
    pub(crate) fn plain() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: false,
            pedersen: None,
            range_check: None,
            ecdsa: None,
            bitwise: None,
            ec_op: None,
            keccak: None,
            poseidon: None,
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn small() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::default()),
            range_check: Some(RangeCheckInstanceDef::default()),
            ecdsa: Some(EcdsaInstanceDef::default()),
            bitwise: None,
            ec_op: None,
            keccak: None,
            poseidon: None,
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn dex() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::default()),
            range_check: Some(RangeCheckInstanceDef::default()),
            ecdsa: Some(EcdsaInstanceDef::default()),
            bitwise: None,
            ec_op: None,
            keccak: None,
            poseidon: None,
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn recursive() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::new(Some(128))),
            range_check: Some(RangeCheckInstanceDef::default()),
            ecdsa: None,
            bitwise: Some(BitwiseInstanceDef::new(Some(8))),
            ec_op: None,
            keccak: None,
            poseidon: None,
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn starknet() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::new(Some(32))),
            range_check: Some(RangeCheckInstanceDef::new(Some(16))),
            ecdsa: Some(EcdsaInstanceDef::new(Some(2048))),
            bitwise: Some(BitwiseInstanceDef::new(Some(64))),
            ec_op: Some(EcOpInstanceDef::new(Some(1024))),
            keccak: None,
            poseidon: Some(PoseidonInstanceDef::default()),
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn starknet_with_keccak() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::new(Some(32))),
            range_check: Some(RangeCheckInstanceDef::new(Some(16))),
            ecdsa: Some(EcdsaInstanceDef::new(Some(2048))),
            bitwise: Some(BitwiseInstanceDef::new(Some(64))),
            ec_op: Some(EcOpInstanceDef::new(Some(1024))),
            keccak: Some(KeccakInstanceDef::new(Some(2048))),
            poseidon: Some(PoseidonInstanceDef::default()),
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn recursive_large_output() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::new(Some(128))),
            range_check: Some(RangeCheckInstanceDef::default()),
            ecdsa: None,
            bitwise: Some(BitwiseInstanceDef::new(Some(8))),
            ec_op: None,
            keccak: None,
            poseidon: Some(PoseidonInstanceDef::new(Some(8))),
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn recursive_with_poseidon() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::new(Some(256))),
            range_check: Some(RangeCheckInstanceDef::new(Some(16))),
            ecdsa: None,
            bitwise: Some(BitwiseInstanceDef::new(Some(16))),
            ec_op: None,
            keccak: None,
            poseidon: Some(PoseidonInstanceDef::new(Some(64))),
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn all_cairo() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::new(Some(256))),
            range_check: Some(RangeCheckInstanceDef::default()),
            ecdsa: Some(EcdsaInstanceDef::new(Some(2048))),
            bitwise: Some(BitwiseInstanceDef::new(Some(16))),
            ec_op: Some(EcOpInstanceDef::new(Some(1024))),
            keccak: Some(KeccakInstanceDef::new(Some(2048))),
            poseidon: Some(PoseidonInstanceDef::new(Some(256))),
            range_check96: Some(RangeCheckInstanceDef::new(Some(8))),
            #[cfg(feature = "mod_builtin")]
            add_mod: Some(ModInstanceDef::new(Some(128), 1, 96)),
            #[cfg(feature = "mod_builtin")]
            mul_mod: Some(ModInstanceDef::new(Some(256), 1, 96)),
            #[cfg(not(feature = "mod_builtin"))]
            add_mod: None,
            #[cfg(not(feature = "mod_builtin"))]
            mul_mod: None,
            dojo: Some(DojoInstanceDef::new(Some(8))),
        }
    }

    pub(crate) fn all_solidity() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::default()),
            range_check: Some(RangeCheckInstanceDef::default()),
            ecdsa: Some(EcdsaInstanceDef::default()),
            bitwise: Some(BitwiseInstanceDef::default()),
            ec_op: Some(EcOpInstanceDef::default()),
            keccak: None,
            poseidon: None,
            range_check96: None,
            add_mod: None,
            mul_mod: None,
            dojo: None,
        }
    }

    pub(crate) fn dynamic() -> BuiltinsInstanceDef {
        BuiltinsInstanceDef {
            output: true,
            pedersen: Some(PedersenInstanceDef::new(None)),
            range_check: Some(RangeCheckInstanceDef::new(None)),
            ecdsa: Some(EcdsaInstanceDef::new(None)),
            bitwise: Some(BitwiseInstanceDef::new(None)),
            ec_op: Some(EcOpInstanceDef::new(None)),
            keccak: None,
            poseidon: None,
            range_check96: None,
            #[cfg(feature = "mod_builtin")]
            add_mod: Some(ModInstanceDef::new(None, 1, 96)),
            #[cfg(feature = "mod_builtin")]
            mul_mod: Some(ModInstanceDef::new(None, 1, 96)),
            #[cfg(not(feature = "mod_builtin"))]
            add_mod: None,
            #[cfg(not(feature = "mod_builtin"))]
            mul_mod: None,
            dojo: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_builtins_plain() {
        let builtins = BuiltinsInstanceDef::plain();
        assert!(!builtins.output);
        assert!(builtins.pedersen.is_none());
        assert!(builtins.range_check.is_none());
        assert!(builtins.ecdsa.is_none());
        assert!(builtins.bitwise.is_none());
        assert!(builtins.ec_op.is_none());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_builtins_small() {
        let builtins = BuiltinsInstanceDef::small();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_some());
        assert!(builtins.bitwise.is_none());
        assert!(builtins.ec_op.is_none());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_builtins_dex() {
        let builtins = BuiltinsInstanceDef::dex();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_some());
        assert!(builtins.bitwise.is_none());
        assert!(builtins.ec_op.is_none());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_none());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_builtins_recursive() {
        let builtins = BuiltinsInstanceDef::recursive();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_none());
        assert!(builtins.bitwise.is_some());
        assert!(builtins.ec_op.is_none());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_none());
    }

    #[test]
    fn get_builtins_starknet() {
        let builtins = BuiltinsInstanceDef::starknet();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_some());
        assert!(builtins.bitwise.is_some());
        assert!(builtins.ec_op.is_some());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_some());
    }

    #[test]
    fn get_builtins_starknet_with_keccak() {
        let builtins = BuiltinsInstanceDef::starknet_with_keccak();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_some());
        assert!(builtins.bitwise.is_some());
        assert!(builtins.ec_op.is_some());
        assert!(builtins.keccak.is_some());
        assert!(builtins.poseidon.is_some());
    }

    #[test]
    fn get_builtins_recursive_large_output() {
        let builtins = BuiltinsInstanceDef::recursive_large_output();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_none());
        assert!(builtins.bitwise.is_some());
        assert!(builtins.ec_op.is_none());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_some());
    }

    #[test]
    fn get_builtins_all_cairo() {
        let builtins = BuiltinsInstanceDef::all_cairo();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_some());
        assert!(builtins.bitwise.is_some());
        assert!(builtins.ec_op.is_some());
        assert!(builtins.keccak.is_some());
        assert!(builtins.poseidon.is_some());
    }

    #[test]
    fn get_builtins_all_solidity() {
        let builtins = BuiltinsInstanceDef::all_solidity();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_some());
        assert!(builtins.bitwise.is_some());
        assert!(builtins.ec_op.is_some());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_none());
    }

    #[test]
    fn get_builtins_dynamic() {
        let builtins = BuiltinsInstanceDef::dynamic();
        assert!(builtins.output);
        assert!(builtins.pedersen.is_some());
        assert!(builtins.range_check.is_some());
        assert!(builtins.ecdsa.is_some());
        assert!(builtins.bitwise.is_some());
        assert!(builtins.ec_op.is_some());
        assert!(builtins.keccak.is_none());
        assert!(builtins.poseidon.is_none());
    }
}
