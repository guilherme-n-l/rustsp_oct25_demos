//! Simple Rust module for the Linux Kernel for demonstration

use kernel::prelude::*; // Basic utilities

module! {
    type: RustSPModule,
    name: "rust_sp",
    authors: ["Rust SP Team"],
    description: "Sample demonstration module",
    license: "GPL",
}

struct RustSPModule {
    greeting: &'static str,
    farewell: &'static str,
}

impl kernel::Module for RustSPModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let this = RustSPModule {
            greeting: "Hello from the kernel. With love, Rust",
            farewell: "Goodbye. Thank you for participating",
        };
        pr_info!("{:?}\n", this.greeting);
        Ok(this)
    }
}

impl Drop for RustSPModule {
    fn drop(&mut self) {
        pr_info!("{:?}\n", self.farewell);
    }
}
