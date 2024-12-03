use std::fmt::Display;

use super::terminal::TerminalLogger;

#[derive(Debug, Clone)]
pub enum PackageLogs {
    Welcome,
    Desc,
    Installed,
    UnInstalled,
    InstallErr(String),
    Init,
    Start,
    Confirm,
    PackageResourced,
    Error,
    Configing
}

impl Display for PackageLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageLogs::Welcome => f.write_str("🥳 Welcome to use gpiler packager!"),
            PackageLogs::Desc => f.write_str(DESC),
            PackageLogs::Installed => {
                f.write_str("✅ cargo-packager has been installed successfully!")
            }
            PackageLogs::UnInstalled => f.write_str("❗️ cargo-packager has not been installed!"),
            PackageLogs::Start => f.write_str("📦 Package is being started"),
            PackageLogs::Confirm => {
                f.write_str("🎉 Congratulations! The current project has been packaged!")
            }
            PackageLogs::InstallErr(s) => {
                f.write_fmt(format_args!("❌ Install cargo-packager failed!\n: {}", s))
            },
            PackageLogs::Init => f.write_str(INIT_MSG),
            PackageLogs::PackageResourced => f.write_str("🎉 Package resources has been generated!\nYou can see a `Packager.toml` for packaging settings and a `packaging dir` for package resources!"),
            PackageLogs::Error => f.write_str("❌ Package failed! Please check the error message!"),
            PackageLogs::Configing => f.write_str("🚀 Processing packaged resources..."),
        }
    }
}

impl PackageLogs {
    pub fn terminal(&self) -> TerminalLogger {
        TerminalLogger {
            output: self.to_string(),
        }
    }
}

const DESC: &str = r#"
🔸 gpiler will check `cargo-packager` is installed or not
🔸 gpiler will help you install `cargo-packager`
🔸 about configuration: https://docs.crabnebula.dev/packager/
"#;

const INIT_MSG: &str = r#"
🔸 init: gpiler will help you init an easy config for packaging
🔸 skip: gpiler will directly run package by configurations
❗️ If you want to define more details, please modify the Packager.toml yourself
"#;
