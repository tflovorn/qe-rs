use std::path::PathBuf;

/// # TODO
///
/// Add `spin_component` to support `CollinearPolarized` spins.
pub struct Input {
    pub prefix: Option<String>,
    pub out_dir: Option<PathBuf>,
    pub filband: Option<PathBuf>,
    pub lsym: bool,
}
