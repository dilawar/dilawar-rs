/// Add to to PATH. If `append` is false, add at the front.
pub fn add_to_path(path: &str, append: bool) -> anyhow::Result<()> {
    use anyhow::Context;
    use std::env;
    use std::path::PathBuf;

    let paths = env::var_os("PATH").context("empty PATH")?;
    let mut paths = env::split_paths(&paths).collect::<Vec<_>>();
    if append {
        paths.push(PathBuf::from(path));
    } else {
        paths.insert(0, PathBuf::from(path));
    }

    let new_path = env::join_paths(paths)?;
    env::set_var("PATH", new_path);

    Ok(())
}
