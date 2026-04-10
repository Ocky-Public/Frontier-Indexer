// This module provides the MoveStruct trait for handling Move structs and event types.

use serde::Serialize;
use std::str::FromStr;
use sui_sdk_types::{Address, Identifier, StructTag};

use crate::{get_module_type, AppEnv, ModuleType};

// Trait for Move structs that can be matched against event types
pub trait MoveStruct: Serialize {
    // Event types matching constants
    const MODULE: &'static str;
    const NAME: &'static str;
    const TYPE_PARAMS: &'static [&'static str] = &[];

    // Get the list of acceptable package addressses for this event tyoe base on environment
    fn acceptable_package_addresses(env: AppEnv) -> Result<Vec<Address>, String> {
        get_package_addresses_for_module(Self::MODULE, env)
    }

    // Check if a struct tag matches this event type from any supported package version
    fn matches_event_type(
        event_type: &move_core_types::language_storage::StructTag,
        env: AppEnv,
    ) -> bool {
        use move_core_types::account_address::AccountAddress;

        // Get all possible struct types for this event
        let all_struct_types = Self::get_all_struct_types(env);

        // Check if the event type matches any of the generated struct types
        // NOTE: We intentionally ignore type_params.len() because events may have phantom/generic type parameters
        // that don't affect the actual event structure
        all_struct_types.iter().any(|struct_type| {
            event_type.address == AccountAddress::new(*struct_type.address.inner())
                && event_type.module.as_str() == struct_type.module.as_str()
                && event_type.name.as_str() == struct_type.name.as_str()
        })
    }

    // Generate all possible struct types for this event across all supported package versions
    fn get_all_struct_types(env: AppEnv) -> Vec<StructTag> {
        let acceptable_address = match Self::acceptable_package_addresses(env) {
            Ok(addresses) => addresses,
            Err(_) => return Vec::new(),
        };
        let mut struct_types = Vec::new();

        for address in acceptable_address {
            let struct_tag = StructTag {
                address: (*address.inner()).into(),
                module: Identifier::from_str(Self::MODULE).unwrap(),
                name: Identifier::from_str(Self::NAME).unwrap(),
                type_params: Self::TYPE_PARAMS
                    .iter()
                    .map(|param| {
                        sui_sdk_types::TypeTag::Struct(Box::new(StructTag {
                            address: (*address.inner()).into(),
                            module: Identifier::from_str(Self::MODULE).unwrap(),
                            name: Identifier::from_str(param).unwrap(),
                            type_params: Vec::new(),
                        }))
                    })
                    .collect(),
            };

            struct_types.push(struct_tag);
        }

        struct_types
    }
}

// Generic helper function that reads package addresses from lib.rs at runtime
pub fn get_package_addresses_for_module(module: &str, env: AppEnv) -> Result<Vec<Address>, String> {
    match get_module_type(module) {
        ModuleType::App => {
            let app_packages = crate::get_app_package_addresses(env);
            let mut addresses = Vec::new();

            for addr_str in app_packages {
                if let Ok(addr) = parse_address_from_hex(addr_str) {
                    addresses.push(addr);
                }
            }

            Ok(addresses)
        }
        ModuleType::World => {
            let world_packages = crate::get_world_package_addresses(env);
            let mut addresses = Vec::new();

            for addr_str in world_packages {
                if let Ok(addr) = parse_address_from_hex(addr_str) {
                    addresses.push(addr);
                }
            }

            Ok(addresses)
        }
        ModuleType::Sui => {
            const SUI_SYSTEM_ADDRESS: &str =
                "0000000000000000000000000000000000000000000000000000000000000002";

            if let Ok(addr) = parse_address_from_hex(SUI_SYSTEM_ADDRESS) {
                Ok(vec![addr])
            } else {
                Err("Failed to parse SUI system address".to_string())
            }
        }
        ModuleType::Unknown => Err(format!("Unknown module: {}", module)),
    }
}

// Helper function to parse hex string to addresses
fn parse_address_from_hex(hex_str: &str) -> Result<Address, String> {
    // Remove 0x prefix if present
    let hex_str = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };

    // Pase hex string to bytes
    let bytes = hex::decode(hex_str).map_err(|e| format!("Failed to decode hex: {}", e))?;

    if bytes.len() != 32 {
        return Err(format!("Expected 32 bytes, got {}", bytes.len()));
    }

    let mut addr_bytes = [0u8; 32];
    addr_bytes.copy_from_slice(&bytes);

    Ok(Address::new(addr_bytes))
}
