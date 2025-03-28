use std::fmt::Display;

use toml_edit::{value, Item, Table};

use super::{Position, Size};

/// # DmgConfig
/// The Apple Disk Image (.dmg) configuration.
#[derive(Debug, Clone, Default)]
pub struct DmgConfig {
    /// Position of application folder on window.
    pub app_folder_position: Option<Position>,
    /// Position of application file on window.
    pub app_position: Option<Position>,
    /// Image to use as the background in dmg file. Accepted formats: png/jpg/gif.
    pub background: Option<String>,
    /// Position of volume window on screen.
    pub window_position: Option<Position>,
    /// Size of volume window.
    pub window_size: Option<Size>,
}

impl Display for DmgConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

impl From<&DmgConfig> for Item {
    fn from(v: &DmgConfig) -> Self {
        let mut table = Table::new();

        if let Some(app_folder_position) = v.app_folder_position.as_ref() {
            table.insert("app-folder-position", value(app_folder_position));
        }

        if let Some(app_position) = v.app_position.as_ref() {
            table.insert("app-position", value(app_position));
        }

        if let Some(background) = v.background.as_ref() {
            table.insert("background", value(background));
        }

        if let Some(window_position) = v.window_position.as_ref() {
            table.insert("window-position", value(window_position));
        }

        if let Some(window_size) = v.window_size.as_ref() {
            table.insert("window-size", value(window_size));
        }
        table.set_implicit(false);
        toml_edit::Item::Table(table)
    }
}
