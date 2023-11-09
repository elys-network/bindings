#[allow(deprecated)]
mod multitest;

#[cfg(test)]
#[allow(deprecated)]
mod tests;

pub use multitest::{ElysApp, ElysAppWrapped, ElysModule};
