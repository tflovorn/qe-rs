use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::File;
use pw::input;
use pw::input::{Calculation, DiskIO, Efield, Ibrav, Input, LatticeDirection, Occupations,
                RestartMode, SpinType};

pub fn make_input_file(input: &Input) -> Result<String, Error> {
    input::validate(&input)?;

    let control = make_control(&input)?;
    let system = make_system(&input);

    let input_text = [control, system].join("\n");

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
        lines.push(format!("    eamp={},", eamp));
    };

    lines.push(String::from(" /"));
    lines.join("\n")
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
