use anyhow::Result;
use std::env::current_dir;
use std::path::PathBuf;

pub fn get_current_statics_path(path: &str) -> Result<PathBuf> {
    let mut dir = current_dir()?;
    dir.push(path);
    Ok(dir)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn reminder_test() {
//         let input =get_current_statics_path("path");
//         assert!(input.is_ok());
//     }
// }
