//! Handler for the `version` command.

/// Print version information.
pub fn handle_version() {
    println!(
        "rumdl {} (combined-markdown-formatting-and-code-block-tools branch)",
        env!("CARGO_PKG_VERSION")
    );
}
