#[allow(deprecated)]
mod multitest;

#[cfg(test)]
#[allow(deprecated)]
mod tests;

pub use multitest::{
    ElysApp, ElysAppWrapped, ElysModule, ACCOUNT, ASSET_INFO, BLOCK_TIME, LAST_MODULE_USED,
    PERPETUAL_OPENED_POSITION, PRICES,
};
