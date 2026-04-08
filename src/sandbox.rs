//! Sandbox mode support for the indexer.
//!
//! Provides a write-once global override for package addresses, allowing the indexer
//! to use CLI-provided package IDs instead of the hardcoded mainnet/testnet constants.
//! Call [`init_package_override`] once at startup before any handler processes checkpoints.

use std::sync::OnceLock;

#[derive(Debug)]
struct PackageOverride {
    app: &'static [&'static str], // Application IDs
    world: &'static [&'static str],
}

static PACKAGE_OVERRIDE: OnceLock<PackageOverride> = OnceLock::new();

/// Initialize custom package addresses for sandbox mode.
/// Must be called exactly once at startup, before any handler processes checkpoints.
///
/// Uses `Box::leak` to promote dynamic strings to `'static` lifetime —
/// acceptable because the indexer process needs them for its entire lifetime.
pub fn init_package_override(app: Vec<String>, world: Vec<String>) {
    let app: Vec<&'static str> = app
        .into_iter()
        .map(|s| &*Box::leak(s.into_boxed_str()))
        .collect();

    let world: Vec<&'static str> = world
        .into_iter()
        .map(|s| &*Box::leak(s.into_boxed_str()))
        .collect();

    PACKAGE_OVERRIDE
        .set(PackageOverride {
            app: Box::leak(app.into_boxed_slice()),
            world: Box::leak(world.into_boxed_slice()),
        })
        .expect("init_package_override must only be called once");
}

/// Returns sandbox app package addresses if override is active.
pub(crate) fn app_packages() -> Option<&'static [&'static str]> {
    PACKAGE_OVERRIDE.get().map(|o| o.app)
}

/// Return sandbox world package addresses if override is active.
pub(crate) fn world_packages() -> Option<&'static [&'static str]> {
    PACKAGE_OVERRIDE.get().map(|o| o.world)
}
