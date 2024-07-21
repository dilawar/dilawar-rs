pub mod download;
pub use download::*;

pub mod script;
pub use script::*;


/// Add to to PATH. If `append` is false, add at the front.
pub fn add_to_path(path: &str, append: bool) -> anyhow::Result<()> {
    use std::env;
    use std::path::PathBuf;
    use anyhow::Context;
  
    let path = env::var_os("PATH").context("empty PATH")?;
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    if append {
        paths.push(PathBuf::from(path));
    } else {
        path.insert(0, PathBuf::from(path));
    }
  
    let new_path = env::join_paths(paths)?;
    env::set_var("PATH", &new_path);
  
    Ok(())
}
