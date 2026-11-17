//! SigmaOS-regional-format-management — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_regional_format_management_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-regional-format-management: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-regional-format-management: ok");
}
