use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeName {
    pub name: String,
}

// SUI Module
pub mod sui {
    pub mod sui {
        use crate::traits::MoveStruct;
        use serde::{Deserialize, Serialize};
        use sui_sdk_types::Address;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SUI {
            pub id: Address,
        }

        impl MoveStruct for SUI {
            const MODULE: &'static str = "sui";
            const NAME: &'static str = "SUI";
        }
    }
}
