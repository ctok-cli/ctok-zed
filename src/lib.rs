use std::env;
use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "@ctok/mcp";
const SERVER_PATH: &str = "node_modules/@ctok/mcp/dist/server.js";

struct CtokExtension;

impl zed::Extension for CtokExtension {
    fn new() -> Self {
        CtokExtension
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if installed_version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let node_path = zed::node_binary_path()?;
        let server_path = env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: node_path,
            args: vec![server_path],
            env: Default::default(),
        })
    }
}

zed::register_extension!(CtokExtension);
