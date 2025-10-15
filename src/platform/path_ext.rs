#![allow(dead_code)]

use std::path::Path;
use std::path::PathBuf;

use normalize_path::NormalizePath;

use super::os_string_ext::OsStringExt;

pub trait PathExt {
    fn try_parent(&self) -> std::io::Result<&Path>;
    fn try_file_name(&self) -> std::io::Result<String>;
    fn try_file_stem(&self) -> std::io::Result<String>;
    fn try_to_string(&self) -> std::io::Result<String>;
    fn to_absolute(&self) -> std::io::Result<PathBuf>;
    fn find_ancestor_file<S: AsRef<Path>>(
        &self,
        file_name: S,
    ) -> std::io::Result<Vec<PathBuf>>;
}

impl PathExt for PathBuf {
    fn try_parent(&self) -> std::io::Result<&Path> {
        match self.parent() {
            Some(path) => Ok(path),
            None => Err(std::io::Error::other("Unable to find parent")),
        }
    }

    fn try_file_name(&self) -> std::io::Result<String> {
        match self.file_name() {
            Some(v) => Ok(v.try_to_string()?),
            None => Err(std::io::Error::other("Cannot get file name")),
        }
    }

    fn try_file_stem(&self) -> std::io::Result<String> {
        match self.file_stem() {
            Some(v) => Ok(v.try_to_string()?),
            None => Err(std::io::Error::other("Cannot get file stem")),
        }
    }

    fn try_to_string(&self) -> std::io::Result<String> {
        match self.to_str() {
            Some(v) => Ok(v.to_string()),
            None => Err(std::io::Error::other("Cannot convert Path to string")),
        }
    }

    fn to_absolute(&self) -> std::io::Result<PathBuf> {
        if self.is_absolute() {
            Ok(self.normalize())
        } else {
            let cwd = std::env::current_dir()?;
            Ok(cwd.join(self).normalize())
        }
    }

    fn find_ancestor_file<S: AsRef<Path>>(
        &self,
        file_name: S,
    ) -> std::io::Result<Vec<PathBuf>> {
        let file_name = file_name.as_ref();
        let mut found = vec![];

        let start_dir = match self.is_dir() {
            true => self,
            false => self.try_parent()?,
        };

        let mut current = start_dir.to_path_buf();

        loop {
            let possible = current.join(file_name);

            if std::fs::exists(&possible)? {
                found.push(possible)
            }

            let Some(next) = current.parent() else {
                break;
            };

            current = next.to_path_buf();
        }

        Ok(found)
    }
}

impl PathExt for Path {
    fn try_parent(&self) -> std::io::Result<&Path> {
        match self.parent() {
            Some(path) => Ok(path),
            None => Err(std::io::Error::other("Unable to find parent")),
        }
    }

    fn try_file_name(&self) -> std::io::Result<String> {
        match self.file_name() {
            Some(v) => Ok(v.try_to_string()?),
            None => Err(std::io::Error::other("Cannot get file name")),
        }
    }

    fn try_file_stem(&self) -> std::io::Result<String> {
        match self.file_stem() {
            Some(v) => Ok(v.try_to_string()?),
            None => Err(std::io::Error::other("Cannot get file stem")),
        }
    }

    fn try_to_string(&self) -> std::io::Result<String> {
        match self.to_str() {
            Some(v) => Ok(v.to_string()),
            None => Err(std::io::Error::other("Cannot convert Path to string")),
        }
    }

    fn to_absolute(&self) -> std::io::Result<PathBuf> {
        if self.is_absolute() {
            Ok(self.normalize())
        } else {
            let cwd = std::env::current_dir()?;
            Ok(cwd.join(self).normalize())
        }
    }

    fn find_ancestor_file<S: AsRef<Path>>(
        &self,
        file_name: S,
    ) -> std::io::Result<Vec<PathBuf>> {
        let file_name = file_name.as_ref();
        let mut found = vec![];

        let start_dir = match self.is_dir() {
            true => self,
            false => self.try_parent()?,
        };

        let mut current = start_dir.to_path_buf();

        loop {
            let possible = current.join(file_name);

            if std::fs::exists(&possible)? {
                found.push(possible)
            }

            let Some(next) = current.parent() else {
                break;
            };

            current = next.to_path_buf();
        }

        Ok(found)
    }
}
