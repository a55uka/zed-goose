use zed_extension_api::register_extension;

// Highlighting alone needs no extension code; the crate only satisfies the
// extension packaging, and is the place to wire `goose lsp` later.
struct GooseExtension;

impl zed_extension_api::Extension for GooseExtension {
    fn new() -> Self {
        GooseExtension
    }
}

register_extension!(GooseExtension);
