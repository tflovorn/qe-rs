use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::File;
use pw::input;
use pw::input::{generate_uniform_kpoints, Calculation, Diagonalization, DiskIO, Efield, Ibrav,
                Input, KPoints, LatticeDirection, LatticeUnits, Occupations,
                PositionCoordinateType, RestartMode, SpinType, StartingWfc};

pub fn make_input_file(input: &Input) -> Result<String, Error> {
    input::validate(&input)?;

    let control = make_control(&input)?;
    let system = make_system(&input);
    let electrons = make_electrons(&input);
    let species = make_species(&input);
    let cell = make_cell(&input);
    let positions = make_positions(&input);
    let k_points = make_k_points(&input);

    let mut input_sections = vec![control, system, electrons, species];

    if let Some(cell) = cell {
        input_sections.push(cell)
    }

    input_sections.extend(vec![positions, k_points]);

    let input_text = input_sections.join("\n");

    return Ok(input_text);
}

fn make_control(input: &Input) -> Result<String, Error> {
    let mut lines = Vec::new();
    lines.push(String::from(" &control"));

    lines.push(format!("    calculation='{}',", input.calculation.value()));

    let control = &input.control;

    if let Some(ref restart_mode) = control.restart_mode {
        lines.push(format!("    restart_mode='{}',", restart_mode.value()))
    }

    if let Some(ref disk_io) = control.disk_io {
        lines.push(format!("    disk_io='{}',", disk_io.value()))
    }

    push_bool_field(&mut lines, "wf_collect", control.wf_collect);

    if let Some(ref pseudo_dir) = control.pseudo_dir {
        let path = pseudo_dir.to_str().ok_or(Error::PseudoDir)?;
        lines.push(format!("    pseudo_dir='{}',", path));
    }

    if let Some(ref out_dir) = control.out_dir {
        let path = out_dir.to_str().ok_or(Error::OutDir)?;
        lines.push(format!("    out_dir='{}',", path));
    }

    if let Some(Efield::TeField { dipfield, .. }) = input.efield {
        push_bool_field(&mut lines, "tefield", Some(true));
        push_bool_field(&mut lines, "dipfield", Some(dipfield));
    }

    if let Some(ref prefix) = control.prefix {
        lines.push(format!("    prefix='{}',", prefix));
    }

    lines.push(String::from(" /"));
    Ok(lines.join("\n"))
}

fn push_bool_field(lines: &mut Vec<String>, name: &str, b: Option<bool>) {
    if let Some(b) = b {
        let val = if b {
            String::from(".true.")
        } else {
            String::from(".false.")
        };

        lines.push(format!("    {}={},", name, val));
    };
}

fn make_system(input: &Input) -> String {
    let mut lines = Vec::new();
    lines.push(String::from(" &system"));

    let system = &input.system;

    lines.push(format!("    ibrav={},", system.ibrav.value()));
    lines.push(format!("    celldm(1)={},", system.alat));

    let nat = input.atomic_positions.coordinates.len();
    lines.push(format!("    nat={},", nat));

    let ntyp = input.species.len();
    lines.push(format!("    ntyp={},", ntyp));

    lines.push(format!("    ecutwfc={},", system.ecutwfc));
    lines.push(format!("    ecutrho={},", system.ecutrho));

    lines.push(format!("    occupations='{}',", system.occupations.value()));

    if let Some(ref spin_type) = system.spin_type {
        match *spin_type {
            SpinType::NonPolarized => {
                lines.push(format!("    nspin=1,"));
            }
            SpinType::CollinearPolarized => {
                lines.push(format!("    nspin=2,"));
            }
            SpinType::Noncollinear { spin_orbit } => {
                lines.push(format!("    noncolin=.true.,"));
                push_bool_field(&mut lines, "lspinorb", Some(spin_orbit));
            }
        };
    };

    if let Some(Efield::TeField {
        ref edir,
        emaxpos,
        eopreg,
        eamp,
        ..
    }) = input.efield
    {
        lines.push(format!("    edir={},", edir.value()));
        lines.push(format!("    emaxpos={},", emaxpos));
        lines.push(format!("    eopreg={},", eopreg));
        lines.push(format!("    eamp={:e},", eamp));
    };

    lines.push(String::from(" /"));
    lines.join("\n")
}

fn make_electrons(input: &Input) -> String {
    let mut lines = Vec::new();
    lines.push(String::from(" &electrons"));

    let electrons = &input.electrons;

    if let Some(ref startingwfc) = electrons.startingwfc {
        lines.push(format!("    startingwfc='{}',", startingwfc.value()));
    };

    if let Some(ref diagonalization) = electrons.diagonalization {
        lines.push(format!(
            "    diagonalization='{}',",
            diagonalization.value()
        ));
    };

    match input.calculation {
        Calculation::Scf { conv_thr } => {
            lines.push(format!("    conv_thr={:e},", conv_thr));
        }
        Calculation::Nscf { diago_thr_init, .. } | Calculation::Bands { diago_thr_init, .. } => {
            lines.push(format!("    diago_thr_init={:e},", diago_thr_init));
        }
    }

    lines.push(String::from(" /"));
    lines.join("\n")
}

fn make_species(input: &Input) -> String {
    let mut lines = Vec::new();
    lines.push(String::from("ATOMIC_SPECIES"));

    for species in &input.species {
        lines.push(format!(
            " {} {} {}",
            species.label, species.mass, species.pseudopotential_filename
        ));
    }

    lines.join("\n")
}

fn make_cell(input: &Input) -> Option<String> {
    match &input.system.ibrav {
        &Ibrav::Free(ref cell) => {
            let mut lines = Vec::new();

            lines.push(format!("CELL_PARAMETERS {}", cell.units.value()));

            for latvec in cell.cell.iter() {
                lines.push(format!(" {} {} {}", latvec[0], latvec[1], latvec[2]));
            }

            Some(lines.join("\n"))
        }
    }
}

fn make_positions(input: &Input) -> String {
    let mut lines = Vec::new();

    let positions = &input.atomic_positions;

    lines.push(format!(
        "ATOMIC_POSITIONS {}",
        positions.coordinate_type.value()
    ));

    for coord in &positions.coordinates {
        if let Some(if_pos) = coord.if_pos {
            let r = coord.r;
            lines.push(format!(
                " {} {} {} {} {}",
                coord.species,
                r[0],
                r[1],
                r[2],
                render_bool_list(if_pos)
            ));
        } else {
            let r = coord.r;
            lines.push(format!(" {} {} {} {}", coord.species, r[0], r[1], r[2]));
        }
    }

    lines.join("\n")
}

fn make_k_points(input: &Input) -> String {
    let mut lines = Vec::new();
    lines.push(format!("K_POINTS {}", input.k_points.value()));

    match &input.k_points {
        &KPoints::Crystal(ref k_points) => {
            lines.push(format!("{}", k_points.len()));

            for kw in k_points {
                lines.push(format!("{} {} {} {}", kw[0], kw[1], kw[2], kw[3]));
            }
        }
        &KPoints::CrystalUniform(nk) => {
            let k_points = generate_uniform_kpoints(nk);
            let weight = 1.0 / (k_points.len() as f64);

            lines.push(format!("{}", k_points.len()));

            for k in k_points {
                lines.push(format!("{} {} {} {}", k[0], k[1], k[2], weight));
            }
        }
        &KPoints::Automatic { nk, sk } => {
            let sk_str = match sk {
                Some(sk) => render_bool_list(sk),
                None => render_bool_list([false, false, false]),
            };
            lines.push(format!("{} {} {} {}", nk[0], nk[1], nk[2], sk_str));
        }
        &KPoints::CrystalBands {
            nk_per_panel,
            ref panel_bounds,
        } => {
            lines.push(format!("{}", panel_bounds.len()));

            for k in panel_bounds {
                lines.push(format!("{} {} {} {}", k[0], k[1], k[2], nk_per_panel));
            }
        }
    }

    lines.join("\n")
}

fn render_bool_list(xs: [bool; 3]) -> String {
    let mut result = Vec::new();

    for x in xs.iter() {
        if *x {
            result.push("1");
        } else {
            result.push("0");
        }
    }

    result.join(" ")
}

pub fn write_input_file<P: AsRef<Path>>(input: &Input, file_path: P) -> Result<(), Error> {
    let input_text = make_input_file(input)?;

    let mut file = File::create(file_path)?;
    file.write_all(input_text.as_bytes())?;

    Ok(())
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)] Input(input::ErrorList),
    #[fail(display = "{}", _0)] Io(#[cause] io::Error),
    #[fail(display = "`pseudo_dir` is not valid UTF-8")] PseudoDir,
    #[fail(display = "`out_dir` is not valid UTF-8")] OutDir,
}

impl From<input::ErrorList> for Error {
    fn from(errs: input::ErrorList) -> Error {
        Error::Input(errs)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

/// A `Field` has a method `value()` which returns its textual representation on the
/// right-hand side of a `field_name = value` expression in the QE input file.
pub trait Field {
    fn value(&self) -> String;
}

impl Field for Calculation {
    fn value(&self) -> String {
        String::from(match *self {
            Calculation::Scf { .. } => "scf",
            Calculation::Nscf { .. } => "nscf",
            Calculation::Bands { .. } => "bands",
        })
    }
}

impl Field for RestartMode {
    fn value(&self) -> String {
        String::from(match *self {
            RestartMode::FromScratch => "from_scratch",
            RestartMode::Restart => "restart",
        })
    }
}

impl Field for DiskIO {
    fn value(&self) -> String {
        String::from(match *self {
            DiskIO::Low => "low",
            DiskIO::Medium => "medium",
            DiskIO::High => "high",
            DiskIO::NoDiskIO => "none",
        })
    }
}

impl Field for LatticeDirection {
    fn value(&self) -> String {
        String::from(match *self {
            LatticeDirection::D1 => "1",
            LatticeDirection::D2 => "2",
            LatticeDirection::D3 => "3",
        })
    }
}

impl Field for Ibrav {
    fn value(&self) -> String {
        String::from(match *self {
            Ibrav::Free(_) => "0",
        })
    }
}

impl Field for Occupations {
    fn value(&self) -> String {
        String::from(match *self {
            Occupations::Smearing(_, _) => "smearing",
            Occupations::Tetrahedra => "tetrahedra",
            Occupations::TetrahedraLin => "tetrahedra_lin",
            Occupations::TetrahedraOpt => "tetrahedra_opt",
            Occupations::Fixed => "fixed",
        })
    }
}

impl Field for StartingWfc {
    fn value(&self) -> String {
        String::from(match *self {
            StartingWfc::Atomic => "atomic",
            StartingWfc::AtomicPlusRandom => "atomic+random",
            StartingWfc::Random => "random",
            StartingWfc::File => "file",
        })
    }
}

impl Field for Diagonalization {
    fn value(&self) -> String {
        String::from(match *self {
            Diagonalization::David => "david",
            Diagonalization::Cg => "cg",
        })
    }
}

impl Field for LatticeUnits {
    fn value(&self) -> String {
        String::from(match *self {
            LatticeUnits::Bohr => "bohr",
            LatticeUnits::Angstrom => "angstrom",
            LatticeUnits::Alat => "alat",
        })
    }
}

impl Field for PositionCoordinateType {
    fn value(&self) -> String {
        String::from(match *self {
            PositionCoordinateType::AlatCartesian => "alat",
            PositionCoordinateType::BohrCartesian => "bohr",
            PositionCoordinateType::AngstromCartesian => "angstrom",
            PositionCoordinateType::Crystal => "crystal",
            PositionCoordinateType::CrystalSG => "crystal_sg",
        })
    }
}

impl Field for KPoints {
    fn value(&self) -> String {
        String::from(match *self {
            KPoints::Crystal(_) | KPoints::CrystalUniform(_) => "crystal",
            KPoints::Automatic { .. } => "automatic",
            KPoints::CrystalBands { .. } => "crystal_b",
        })
    }
}
