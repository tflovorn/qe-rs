use std::path::PathBuf;

/// # TODO
///
/// Add `spin_component` to support `CollinearPolarized` spins. Ensure that this is set only
/// for this type of spins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub prefix: Option<String>,
    pub out_dir: Option<PathBuf>,
    pub filband: Option<PathBuf>,
    pub lsym: bool,
}
