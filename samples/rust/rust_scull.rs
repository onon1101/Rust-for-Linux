//! scull module in Rust.
use kernel::prelude::*;

module! {
    type: Scull,
    name: "scull",
    license: "GPL",
}

struct Scull;

impl kernel::Module for Scull {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello from scull in rust-next");
        Ok(Scull)
    }
}
