use std::env::{join_paths, var};
use std::ffi::OsString;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

const PROGRAM_FILES_X86_ENV_KEY: &str = "ProgramFiles(x86)";
const STEAM_DIR: &str = "Steam";
const READY_OR_NOT_DIR: &str = "steamapps/common/Ready Or Not";
const PAKS_DIR: &str = "ReadyOrNot/Content/Paks";

pub struct Mod {
    name: String,
    dir: OsString,
}

impl From<OsString> for Mod {
    fn from(dir: OsString) -> Self {
        todo!()
    }
}

pub struct ModsIo {
    dir: OsString,
}

impl ModsIo {
    /// Crates a new ModsIo instance from a given Steam directory
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors
    pub fn new(steam_dir: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self {
            dir: join_paths([
                PathBuf::from(steam_dir.as_ref()),
                PathBuf::from(READY_OR_NOT_DIR),
                PathBuf::from(PAKS_DIR),
                PathBuf::from("mod.io"),
            ])?,
        })
    }

    /// Crates a new ModsIo instance from the default Steam directory
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors
    pub fn try_default() -> anyhow::Result<Self> {
        Self::new(join_paths([
            PathBuf::from(var(PROGRAM_FILES_X86_ENV_KEY)?.as_str()),
            PathBuf::from(STEAM_DIR),
        ])?)
    }

    /// Returns the mods directory
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors
    pub fn mods_dir(&self) -> anyhow::Result<OsString> {
        Ok(join_paths([self.top_dir()?.path(), PathBuf::from("mods")])?)
    }

    /// Yields the installed mods
    /// # Errors
    /// Returns an [`anyhow::Error`] on errors
    pub fn mods(&self) -> anyhow::Result<impl Iterator<Item = Mod> + '_> {
        Ok(read_dir(self.mods_dir()?)?
            .filter_map(Result::ok)
            .filter(|dir_entry| {
                dir_entry
                    .file_type()
                    .map_or(false, |file_type| file_type.is_dir())
            })
            .filter_map(|dir_entry| {
                join_paths([PathBuf::from(self.dir.as_os_str()), dir_entry.path()]).ok()
            })
            .map(Mod::from))
    }

    fn top_dir(&self) -> anyhow::Result<DirEntry> {
        read_dir(self.dir.as_os_str())?
            .find(|inode| {
                if let Ok(dir_entry) = inode {
                    if let Ok(file_type) = dir_entry.file_type() {
                        return file_type.is_dir();
                    }
                }
                false
            })
            .map_or_else(
                || Err(anyhow::Error::msg("subdir not found.")),
                |result| result.map_err(anyhow::Error::from),
            )
    }
}
