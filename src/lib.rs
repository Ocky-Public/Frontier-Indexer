use std::sync::Arc;
use url::Url;

use crate::models::system::table_registry::TableRegistry;

pub mod handlers;
pub mod models;
pub mod sandbox;
pub mod schema;

pub const NOT_MAINNET_PACKAGE: &str = "<not on mainnet>";

pub const MAINNET_REMOTE_STORE_URL: &str = "https://checkpoints.mainnet.sui.io";
pub const TESTNET_REMOTE_STORE_URL: &str = "https://checkpoints.testnet.sui.io";

const MAINNET_PACKAGES: &[&str] = &[];

const MAINNET_WORLD_PACKAGES: &[&str] = &[];

const TESTNET_PACKAGES: &[&str] = &[];

const TESTNET_WORLD_PACKAGES: &[&str] = &[
    "0x2a66a89b5a735738ffa4423ac024d23571326163f324f9051557617319e59d60", // Assets v1
    "0x28b497559d65ab320d9da4613bf2498d5946b2c0ae3597ccfda3072ce127448c", // World v1
    "0xd2fd1224f881e7a705dbc211888af11655c315f2ee0f03fe680fc3176e6e4780", // World v2
];

pub const APP_MODULES: &[&str] = &[];

pub const WORLD_MODULES: &[&str] = &[
    "assets",
    "access",
    "assembly",
    "extension_freeze",
    "gate",
    "storage_unit",
    "turret",
    "character",
    "sig_verify",
    "killmail",
    "network_node",
    "energy",
    "fuel",
    "in_game_id",
    "inventory",
    "location",
    "metadata",
    "status",
    "killmail_registry",
    "object_registry",
    "world",
];

pub const SUI_MODULES: &[&str] = &["sui"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    App,
    World,
    Sui,
    Unknown,
}

pub fn is_app_module(module: &str) -> bool {
    APP_MODULES.contains(&module)
}

pub fn is_world_module(module: &str) -> bool {
    WORLD_MODULES.contains(&module)
}

pub fn is_sui_module(module: &str) -> bool {
    SUI_MODULES.contains(&module)
}

pub fn get_module_type(module: &str) -> ModuleType {
    if is_app_module(module) {
        ModuleType::App
    } else if is_world_module(module) {
        ModuleType::World
    } else if is_sui_module(module) {
        ModuleType::Sui
    } else {
        ModuleType::Unknown
    }
}

pub fn get_all_known_modules() -> Vec<&'static str> {
    let mut modules = Vec::new();
    modules.extend_from_slice(APP_MODULES);
    modules.extend_from_slice(WORLD_MODULES);
    modules.extend_from_slice(SUI_MODULES);
    modules
}

pub fn get_app_modules() -> &'static [&'static str] {
    APP_MODULES
}

pub fn get_world_modules() -> &'static [&'static str] {
    WORLD_MODULES
}

pub fn get_sui_modules() -> &'static [&'static str] {
    SUI_MODULES
}

pub fn is_valid_app_package(package: &str) -> bool {
    package != NOT_MAINNET_PACKAGE
}

pub fn is_valid_world_package(package: &str) -> bool {
    package != NOT_MAINNET_PACKAGE
}

pub fn is_valid_app_packages(packages: &[&str]) -> bool {
    packages.iter().any(|&pkg| is_valid_app_package(pkg))
}

pub fn is_valid_world_packages(packages: &[&str]) -> bool {
    packages.iter().any(|&pkg| is_valid_world_package(pkg))
}

pub fn get_app_package_addresses(env: AppEnv) -> &'static [&'static str] {
    if let Some(app) = sandbox::app_packages() {
        return app;
    }

    match env {
        AppEnv::Mainnet => MAINNET_PACKAGES,
        AppEnv::Testnet => TESTNET_PACKAGES,
    }
}

pub fn get_app_package_address(env: AppEnv) -> Result<&'static str, String> {
    if let Some(app) = sandbox::app_packages() {
        return app
            .first()
            .copied()
            .ok_or_else(|| "No app packages configure in sandbox mode".to_string());
    }

    let packages = get_app_package_addresses(env);

    for &package in packages {
        if is_valid_app_package(package) {
            return Ok(package);
        }
    }

    Err(format!(
        "App package is not supported on {:?}. \
        The app package has not been deployed on this network.",
        env
    ))
}

pub fn get_world_package_addresses(env: AppEnv) -> &'static [&'static str] {
    if let Some(world) = sandbox::world_packages() {
        return world;
    }

    match env {
        AppEnv::Mainnet => MAINNET_WORLD_PACKAGES,
        AppEnv::Testnet => TESTNET_WORLD_PACKAGES,
    }
}

pub fn get_world_package_address(env: AppEnv) -> Result<&'static str, String> {
    if let Some(world) = sandbox::world_packages() {
        return world
            .first()
            .copied()
            .ok_or_else(|| "No world packages configure in sandbox mode".to_string());
    }

    let packages = get_world_package_addresses(env);

    for &package in packages {
        if is_valid_world_package(package) {
            return Ok(package);
        }
    }

    Err(format!(
        "World package is not supported on {:?}. \
        The world package has not been deployed on this network.",
        env
    ))
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum AppEnv {
    Mainnet,
    Testnet,
}

impl AppEnv {
    pub fn remote_store_url(&self) -> Url {
        let url = match self {
            AppEnv::Mainnet => MAINNET_REMOTE_STORE_URL,
            AppEnv::Testnet => TESTNET_REMOTE_STORE_URL,
        };
        Url::parse(url).unwrap()
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub env: AppEnv,
    pub tables: Arc<TableRegistry>,
}

impl AppContext {
    /// Get app package addresses for this environment
    pub fn get_app_package_strings(&self) -> Vec<&str> {
        // If sandbox mode is active, both overrides are set together by init_package_override
        // (World may be an empty slice). Use them instead of the hardcoded constants.
        if let Some(app) = sandbox::app_packages() {
            return app.to_vec();
        }

        let app_packages = match self.env {
            AppEnv::Mainnet => MAINNET_PACKAGES,
            AppEnv::Testnet => TESTNET_PACKAGES,
        };

        app_packages.to_vec()
    }

    /// Get world package addresses for this environment
    pub fn get_world_package_strings(&self) -> Vec<&str> {
        // If sandbox mode is active, both overrides are set together by init_package_override
        // (World may be an empty slice). Use them instead of the hardcoded constants.
        if let Some(world) = sandbox::world_packages() {
            return world.to_vec();
        }

        let world_packages = match self.env {
            AppEnv::Mainnet => MAINNET_WORLD_PACKAGES,
            AppEnv::Testnet => TESTNET_WORLD_PACKAGES,
        };

        world_packages.to_vec()
    }

    /// Get all package addresses (App + World) for this environment
    pub fn get_all_package_strings(&self) -> Vec<&str> {
        // If sandbox mode is active, both overrides are set together by init_package_override
        // (World may be an empty slice). Use them instead of the hardcoded constants.
        if let (Some(app), Some(world)) = (sandbox::app_packages(), sandbox::world_packages()) {
            let mut all = app.to_vec();
            all.extend_from_slice(world);
            return all;
        }

        let (app_packages, world_packages) = match self.env {
            AppEnv::Mainnet => (MAINNET_PACKAGES, MAINNET_WORLD_PACKAGES),
            AppEnv::Testnet => (TESTNET_PACKAGES, TESTNET_WORLD_PACKAGES),
        };

        let mut all_packages = app_packages.to_vec();

        // Add margin packages if they're not invalid
        for &world_package in world_packages {
            if world_package != NOT_MAINNET_PACKAGE {
                all_packages.push(world_package);
            }
        }

        all_packages
    }

    pub fn package_ids(&self) -> Vec<sui_types::base_types::ObjectID> {
        use std::str::FromStr;
        use sui_types::base_types::ObjectID;

        self.get_all_package_strings()
            .iter()
            .map(|pkg| ObjectID::from_str(pkg).unwrap())
            .collect()
    }

    pub fn package_addresses(&self) -> Vec<move_core_types::account_address::AccountAddress> {
        use move_core_types::account_address::AccountAddress;
        use std::str::FromStr;

        self.get_all_package_strings()
            .iter()
            .map(|pkg| AccountAddress::from_str(pkg).unwrap())
            .collect()
    }
}
