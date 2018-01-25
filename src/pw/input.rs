use std::path::PathBuf;

/// Representation of the input file for Quantum Espresso 6.2.
///
/// The format is defined with the principle that "only valid states are representable".
/// However, some fields cannot be conveniently fully defined this way within the type system;
/// e.g. some floating-point values must be checked that they are positive.
///
/// Each field in the QE input file has a corresponding field of this struct or its members.
/// Those fields with default values are generally given as `Option<...>` values here.
/// However, those fields which must be specified when another field has a certain value
/// (such as `eamp`, etc. when `tefield = .true.`) are defined to be required in such
/// circumstances. Additionally there is some opinion taken here about which fields really
/// should not be optional (e.g. `ecutrho` is not optional here, since the default value is not
/// correct for ultrasoft pseudopotentials or PAW datasets).
///
/// When specifying one field necessitates specifying another field, those fields which must
/// be specified together are bundled such that specifying only one is not possible.
pub struct Input {
    /// `calculation` specifies the type of calculation (`scf`, `nscf`, etc.) and parameters
    /// specific to that type.
    pub calculation: Calculation,

    pub control: Control,
    pub system: System,

    /// `efield` specifies the sawtooth electric field parameters, which are in both the
    /// `control` and `system` namelists.
    pub efield: Option<Efield>,

    pub electrons: Electrons,
    pub species: Vec<Species>,
    pub cell_parameters: Cell,

    /// For `nscf` and `bands` calculations, the atomic positions are unused: the corresponding
    /// positions from the `scf` calculation are used instead. Here we still require positions to
    /// be specified in these cases in order to derive `nat`.
    pub atomic_positions: Positions,

    pub k_points: KPoints,
}

pub enum Calculation {
    Scf {conv_thr: f64},
    Nscf {diago_thr_init: f64, nbnd: Option<u64>, nosym: Option<bool>},
    Bands {diago_thr_init: f64, nbnd: Option<u64>, nosym: Option<bool>},
}

pub struct Control {
    pub restart_mode: Option<RestartMode>,
    pub disk_io: Option<DiskIO>,
    pub wf_collect: Option<bool>,
    pub pseudo_dir: Option<PathBuf>,
    pub outdir: Option<PathBuf>,
    pub prefix: Option<String>,
}

pub enum RestartMode {
    FromScratch,
    Restart,
}

pub enum DiskIO {
    Low,
    Medium,
    High,
    NoDiskIO,
}

pub struct System {
    pub ibrav: Ibrav,
    pub alat: f64,
    //pub nat: u64, // obtain from positions list
    //pub ntyp: u64, // obtain from species list
    pub ecutwfc: f64,
    pub ecutrho: f64,
    pub occupations: Occupations,
    pub spin_type: Option<SpinType>,
}

/// Bravais lattice settings, given in the order they appear in the QE input description.
/// Each carries the `celldm` parameters required to specify the lattice dimensions,
/// in the order given in the QE input description. Since it is the same for each case,
/// `alat` is given separately and the corresponding `celldm(1)` value is skipped here.
///
/// When a `celldm` value is skipped, such as for the hexagonal case `ibrav = 4` which
/// uses `celldm(1)` and `celldm(3)` but not `celldm(2)`, we do not include a placeholder
/// for the skipped element(s).
///
/// The "traditional crystallographic constants" A, B, C, cosAB, cosAC, cosBC are not
/// supported here; these can be rewritten in terms of `celldm`.
pub enum Ibrav {
    Free,
    //SimpleCubic,
    //Fcc,
    //Bcc,
    //BccSymmetric,
    //Hexagonal(f64),
    //TrigonalRAxisC(f64),
    //TrigonalRAxis111(f64),
    //TetragonalP(f64),
    //TetragonalI(f64),
    //OrthorhombicP(f64, f64),
    //OrthorhombicBco(f64, f64),
    //OrthorhombicBcoAlternate(f64, f64),
    //OrthorhombicFaceCentered(f64, f64),
    //OrthorhombicBodyCentered(f64, f64),
    //MonoclinicPUniqueAxisC(f64, f64, f64),
    //MonoclinicPUniqueAxisB(f64, f64, f64),
    //MonoclinicBaseCentered(f64, f64, f64),
    //Triclinic(f64, f64, f64, f64, f64),
}

pub enum Occupations {
    /// Each possible type of `Smearing` must come with a `degauss` value giving the
    /// size of the smearing.
    Smearing(Smearing, f64),
    Tetrahedra,
    TetrahedraLin,
    TetrahedraOpt,
    Fixed,
    //FromInput,
}

pub enum Smearing {
    Gaussian,
    MethfesselPaxton,
    MarzariVanderbilt,
    FermiDirac,
}

/// Type of the spin representation.
///
/// `NonPolarized` and `CollinearPolarized` are equivalent to `nspin = 1` and `nspin = 2`
/// respectively. `NonCollinear(false)` is equivalent to `noncolin = .true.`, `lspinorb = .false.`.
/// `Noncollinear(true)` is equivalent to `noncolin = .true.`, `lspinorb = .true.`.
pub enum SpinType {
    NonPolarized,
    CollinearPolarized,
    Noncollinear {spin_orbit: bool},
}

pub enum Efield {
    TeField {dipfield: bool, edir: LatticeDirection, emaxpos: f64, eopreg: f64, eamp: f64,},
    //LelField,
}

pub enum LatticeDirection {
    D1, D2, D3,
}

pub struct Electrons {
    pub startingwfc: Option<StartingWfc>,
    pub diagonalization: Option<Diagonalization>,
}

pub enum StartingWfc {
    Atomic,
    AtomicPlusRandom,
    Random,
    File,
}

pub enum Diagonalization {
    David,
    Cg,
}

pub struct Species {
    pub label: String,
    pub mass: f64,
    pub pseudopotential_filename: String,
}

pub struct Cell {
    pub units: LatticeUnits,
    pub cell: [[f64; 3]; 3],
}

pub enum LatticeUnits {
    Bohr,
    Angstrom,
    Alat,
}

pub struct Positions {
    pub coordinate_type: PositionCoordinateType,
    pub coordinates: Vec<AtomCoordinate>,
}

pub enum PositionCoordinateType {
    AlatCartesian,
    BohrCartesian,
    AngstromCartesian,
    Crystal,
    CrystalSG,
}

pub struct AtomCoordinate {
    pub species: String,
    pub r: [f64; 3],
    pub if_pos: Option<[bool; 3]>,
}

pub enum KPoints {
    //TwoPiByACartesian(Vec<[f64; 4]>),
    Crystal(Vec<[f64; 4]>),
    Automatic { nk: [f64; 3], sk: [f64; 3], },
    //Gamma,
    //TwoPiByACartesianBands { nk_per_panel: u64, panel_bounds: Vec<f64; 3]> },
    CrystalBands { nk_per_panel: u64, panel_bounds: Vec<[f64; 3]> },
    //TwoPiByACartesianContour([[f64; 3]; 3]),
    //CrystalContour([[f64; 3]; 3]),
}

/// Some required properties of the `Input` cannot be conveniently encoded in the type system
/// and must be checked at runtime. If any properties do not have the required form, return
/// a corresponding `Error` for each of them; otherwise return `Ok`.
pub fn validate(input: Input) -> Result<(), Vec<InputError>> {
    let mut errs = Vec::new();
    let system = input.system;

    // Lattice constant must be positive.
    if system.alat <= 0.0 {
        errs.push(InputError::LatticeConstant(system.alat));
    }

    // Check that `conv_thr` or `diago_thr_init` are positive.
    match input.calculation {
        Calculation::Scf {conv_thr} => {
            if conv_thr <= 0.0 {
                errs.push(InputError::ConvThr(conv_thr));
            }
        },
        Calculation::Nscf { diago_thr_init, .. } | Calculation::Bands { diago_thr_init, .. } => {
            if diago_thr_init <= 0.0 {
                errs.push(InputError::DiagoThrInit(diago_thr_init));
            }
        }
    }

    // Check that ecutwfc and ecutrho are positive.
    if system.ecutwfc <= 0.0 {
        errs.push(InputError::Ecutwfc(system.ecutwfc));
    }
    if system.ecutrho <= 0.0 {
        errs.push(InputError::Ecutrho(system.ecutrho));
    }

    // TODO - would be very nice to have, but not simple to fit in since we don't
    // have an explicit statement of the type of pseudopotential:
    // Check that ecutrho is consistent with ecutwfc, according to the pseudopotential type.
    // For NC PP, should have ecutrho = 4 * ecutwfc.
    // For US PP and PAW, should have ecutrho \approx (8 to 12) * ecutwfc.
    // Could implement by extracting the pseudopotential header (UPF format).

    // Check that smearing, if present, is positive.
    if let Occupations::Smearing(_, degauss) = system.occupations {
        if degauss <= 0.0 {
            errs.push(InputError::Smearing(degauss));
        }
    }

    // Check that masses are positive.
    for species in input.species {
        if species.mass <= 0.0 {
            errs.push(InputError::Mass(species.label.clone(), species.mass));
        }
    }

    // TODO: Check that cell volume `|(a1 x a2) . a3|` does not vanish.

    // TODO: Does QE complain if `(a1 x a2) . a3` is negative? If so, check that this is positive.

    // TODO: Check that `emaxpos` and `eopreg`, if present, are between 0 and 1.

    // TODO: Check that if a tetrahedron method is used for occupations, the k-point list type
    // is `automatic`.

    // TODO: All species identified in the atom coordinate list should correspond
    // to species given in the species list.

    if errs.len() == 0 {
        Ok(())
    } else {
        Err(errs)
    }
}

#[derive(Fail, Debug)]
pub enum InputError {
    #[fail(display = "Lattice constant `alat` must be positive; got {} instead.", _0)]
    LatticeConstant(f64),
    #[fail(display = "SCF convergence threshold `conv_thr` must be positive; got {} instead.", _0)]
    ConvThr(f64),
    #[fail(display = "Diagonalization convergence threshold `diago_thr_init` must be positive; got {} insead.", _0)]
    DiagoThrInit(f64),
    #[fail(display = "Wavefunction cutoff energy `ecutwfc` must be positive; got {} instead.", _0)]
    Ecutwfc(f64),
    #[fail(display = "Charge density cutoff energy `ecutwfc` must be positive; got {} instead.", _0)]
    Ecutrho(f64),
    #[fail(display = "Smearing value must be positive; got {} instead.", _0)]
    Smearing(f64),
    #[fail(display = "Atomic mass must be positive; for atom {} got {} instead.", _0, _1)]
    Mass(String, f64),
    #[fail(display = "Species {} in coordinate list is not given in species list.", _0)]
    Species(String),
}
