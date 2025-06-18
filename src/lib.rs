// cc
#[cfg(feature = "full")]
pub mod cmake;
#[cfg(feature = "full")]
pub mod commonflags;

pub mod finalize;

/// The list of configurable flags
#[derive(Clone, Copy)]
pub struct LConfig {
    pub disable_crypto: bool,
    pub disable_net: bool,
}

impl Default for LConfig {
    fn default() -> Self {
        Self {
            disable_crypto: true, // Takes too long to build
            disable_net: true, // Takes too long to build
        }
    }
}
