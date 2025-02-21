#[cfg(feature = "type_generation")]
use crate::response;

#[cfg(feature = "type_generation")]
pub fn get_typescript_definitions() -> String {
    [response::get_typescript_definitions()]
        .concat()
        .join("\n\n")
        .lines()
        .filter(|line| !(line.starts_with("import type") || line.starts_with("// This file")))
        .collect::<Vec<&str>>()
        .join("\n")
        .replace("\n\n", "\n")
        .replace("\n\n\n", "\n\n")
        .replace("\n\n\n", "\n\n")
}
