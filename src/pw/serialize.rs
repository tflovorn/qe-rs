use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::File;
use pw::input;
use pw::input::{Calculation, Input, RestartMode};

pub fn make_input_file(input: &Input) -> Result<String, input::ErrorList> {
    input::validate(&input)?;

    let control = make_control(&input);

    let input_text = [control].join("\n");

    return Ok(input_text);
}

fn make_control(input: &Input) -> String {
    let mut lines = Vec::new();
    lines.push(String::from("&control"));

    lines.push(format!(
        "    calculation='{}'",
        match input.calculation {
            Calculation::Scf { .. } => "scf",
            Calculation::Nscf { .. } => "nscf",
            Calculation::Bands { .. } => "bands",
        }
    ));

    if let Some(ref restart_mode) = input.control.restart_mode {
        lines.push(format!(
            "    restart_mode='{}'",
            match restart_mode {
                &RestartMode::FromScratch => "from_scratch",
                &RestartMode::Restart => "restart",
            }
        ))
    }

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
