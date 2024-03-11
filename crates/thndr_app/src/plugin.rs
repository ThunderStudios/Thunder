use crate::App;

/// A plugin adds functionality to the application.
pub trait Plugin {
    /// Builds the plugin and adds it to the application.
    fn build(&self, app: &mut App);
}
