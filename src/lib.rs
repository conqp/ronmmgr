use std::env::var;
use std::fmt::{Display, Formatter};
use std::fs::read_dir;
use std::path::{Path, PathBuf};

const PROGRAM_FILES_X86_ENV_KEY: &str = "ProgramFiles(x86)";
const STEAM_DIR: &str = "Steam";
const READY_OR_NOT_DIR: &str = r"steamapps\common\Ready Or Not";
const PAKS_DIR: &str = r"ReadyOrNot\Content\Paks";

pub struct Mod {
    name: String,
    dir: PathBuf,
}

impl Mod {
    #[must_use]
    pub fn dir_name(&self) -> &str {
        self.dir.to_str().unwrap_or("N/A")
    }
}

impl Display for Mod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\t{}", self.name, self.dir_name())
    }
}

impl From<PathBuf> for Mod {
    fn from(dir: PathBuf) -> Self {
        Self {
            name: "TODO".to_string(),
            dir,
        }
    }
}

pub struct ModsIo {
    dir: PathBuf,
}

impl ModsIo {
    /// Crates a new `ModsIo` instance from a given Steam directory
    pub fn new(steam_dir: impl AsRef<Path>) -> Self {
        Self {
            dir: steam_dir
                .as_ref()
                .join(PathBuf::from(READY_OR_NOT_DIR))
                .join(PathBuf::from(PAKS_DIR))
                .join(PathBuf::from("mod.io")),
        }
    }

    /// Returns the mods directory
    #[must_use]
    pub fn mods_dir(&self) -> Option<PathBuf> {
        self.top_dir()
            .map(|top_dir| top_dir.join(PathBuf::from("mods")))
    }

    /// Yields the installed mods
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors
    pub fn mods(&self) -> anyhow::Result<impl Iterator<Item = Mod> + '_> {
        Ok(read_dir(
            self.mods_dir()
                .ok_or_else(|| anyhow::Error::msg("mods dir not found"))?,
        )?
        .filter_map(Result::ok)
        .filter(|dir_entry| {
            dir_entry
                .file_type()
                .map_or(false, |file_type| file_type.is_dir())
        })
        .map(|dir_entry| Mod::from(self.dir.join(dir_entry.path()))))
    }

    fn top_dir(&self) -> Option<PathBuf> {
        read_dir(self.dir.as_os_str()).ok().and_then(|entries| {
            entries
                .filter(|inode| {
                    if let Ok(dir_entry) = inode {
                        if let Ok(file_type) = dir_entry.file_type() {
                            return file_type.is_dir();
                        }
                    }
                    false
                })
                .find_map(Result::ok)
                .map(|dir_entry| self.dir.join(dir_entry.path()))
        })
    }
}

impl Default for ModsIo {
    fn default() -> Self {
        Self::new(
            PathBuf::from(
                var(PROGRAM_FILES_X86_ENV_KEY)
                    .unwrap_or_else(|_| "C:/Program Files (x86)".to_string())
                    .as_str(),
            )
            .join(PathBuf::from(STEAM_DIR)),
        )
    }
}

impl Display for ModsIo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dir.to_str().unwrap_or("N/A"))
    }
}
