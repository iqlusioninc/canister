//! Canister Abscissa Application

use crate::{commands::CanisterCommand, config::CanisterConfig};
use abscissa_core::{
    application, application::AppCell, config::{self, CfgCell}, trace, Application, FrameworkError,
    StandardPaths,
};

/// Application state
pub static APPLICATION: AppCell<CanisterApplication> = AppCell::new();


/// Canister Application
#[derive(Debug)]
pub struct CanisterApplication {
    /// Application configuration.
    config: CfgCell<CanisterConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
///
/// By default no configuration is loaded, and the framework state is
/// initialized to a default, empty state (no components, threads, etc).
impl Default for CanisterApplication {
    fn default() -> Self {
        Self {
            config: CfgCell::default(),
            state: application::State::default(),
        }
    }
}

impl Application for CanisterApplication {
    /// Entrypoint command for this application.
    type Cmd = CanisterCommand;

    /// Application configuration.
    type Cfg = CanisterConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> config::Reader<CanisterConfig> {
        self.config.read()
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Register all components used by this application.
    ///
    /// If you would like to add additional components to your application
    /// beyond the default ones provided by the framework, this is the place
    /// to do so.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;

        let mut component_registry = self.state.components_mut();
        component_registry.register(components)
    }

    /// Post-configuration lifecycle callback.
    ///
    /// Called regardless of whether config is loaded to indicate this is the
    /// time in app lifecycle when configuration would be loaded if
    /// possible.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        let mut component_registry = self.state.components_mut();
        component_registry.after_config(&config)?;
        self.config.set_once(config);
        Ok(())
    }

    /// Get tracing configuration from command-line options
    fn tracing_config(&self, command: &CanisterCommand) -> trace::Config {
        if command.verbose() {
            trace::Config::verbose()
        } else {
            trace::Config::default()
        }
    }
}
