// cc
#[cfg(feature = "full")]
pub mod cmake;
#[cfg(feature = "full")]
pub mod commonflags;

pub mod finalize;
pub mod prebuilts;

/// The list of configurable flags
#[derive(Clone, Copy)]
pub struct LConfig {
    pub disable_crypto: bool,
    pub disable_net: bool,
    pub disable_native_codegen: bool,
}

impl Default for LConfig {
    fn default() -> Self {
        Self {
            disable_crypto: true, // Takes too long to build
            disable_net: true, // Takes too long to build
            disable_native_codegen: true, // Limits portability when enabled, takes a bit to build
        }
    }
}
