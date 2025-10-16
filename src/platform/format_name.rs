use std::path::PathBuf;

use crate::platform::os_string_ext::OsStringExt;
use crate::platform::path_ext::PathExt;

pub fn format_name(input: &str) -> String {
    if !input.contains("@") {
        let input = PathBuf::from(input);
        if let Some(extension) = input.extension() {
            return format!(
                "{}.{}",
                slugify::slugify(
                    &input.file_stem().unwrap().try_to_string().unwrap(),
                    "",
                    "-",
                    None
                ),
                extension.try_to_string().unwrap()
            );
        }
        return slugify::slugify(&input.try_to_string().unwrap(), "", "-", None);
    }

    let Some((name, version)) = input.split_once("@") else {
        unreachable!();
    };

    let name = slugify::slugify(name, "", "-", None);
    format!("{}@{}", name, version)
}
