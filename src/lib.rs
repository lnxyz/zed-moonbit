use zed_extension_api::{self as zed, Result};

struct MoonBitExtension;

impl zed::Extension for MoonBitExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_name = "moonbit-lsp";
        let path = worktree
            .which(binary_name)
            .ok_or_else(|| format!("{binary_name} executable not found in $PATH"))?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(MoonBitExtension);
