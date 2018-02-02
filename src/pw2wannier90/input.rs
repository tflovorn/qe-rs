use std::path::PathBuf;

/// Field `prefix` which is optional in `pw.x` and `bands.x` input is not optional here.
/// It optional for `pw2wannier90.x`, but the default behavior differs from `pw.x` and `bands.x`.
///
/// # TODO
///
/// Add `spin_component` to support `CollinearPolarized` spins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub prefix: String,
    pub out_dir: Option<PathBuf>,
    pub seedname: String,
    pub write_unk: bool,
    pub write_amn: bool,
    pub write_mmn: bool,
    pub write_spn: bool,
}
