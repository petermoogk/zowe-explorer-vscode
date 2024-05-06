use anyhow::bail;

use crate::util::find_dir_match;

use super::{detect_pkg_mgr, pkg_mgr};

pub fn handle_cmd(args: Vec<String>) -> anyhow::Result<()> {
    let ze_dir = match find_dir_match(&["package.json"]) {
        Ok(d) => match d {
            Some(d) => d,
            None => bail!("Could not find a repo folder containing package.json."),
        },
        Err(_) => bail!("Could not find a repo folder containing package.json."),
    };

    match pkg_mgr(detect_pkg_mgr(&ze_dir)?.as_str())
        .args(args)
        .current_dir(&ze_dir)
        .status()
    {
        Ok(_) => Ok(()),
        Err(e) => bail!(e),
    }
}
