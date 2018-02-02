use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::File;
use serialize_util::push_bool_field;
use bands::input::Input;

pub fn make_input_file(input: &Input) -> Result<String, Error> {
    let mut lines = Vec::new();
    lines.push(String::from(" &bands"));

    if let Some(ref prefix) = input.prefix {
        lines.push(format!("   prefix='{}',", prefix));
    }

    if let Some(ref out_dir) = input.out_dir {
        let path = out_dir.to_str().ok_or(Error::OutDir)?;
        lines.push(format!("   out_dir='{}',", path));
    }

    if let Some(ref filband) = input.filband {
        let path = filband.to_str().ok_or(Error::Filband)?;
        lines.push(format!("   filband='{}',", path));
    }

    push_bool_field(&mut lines, "lsym", Some(input.lsym));

    lines.push(String::from(" /"));
    Ok(lines.join("\n"))
}

pub fn write_input_file<P: AsRef<Path>>(input: &Input, file_path: P) -> Result<(), Error> {
    let input_text = make_input_file(input)?;

    let mut file = File::create(file_path)?;
    file.write_all(input_text.as_bytes())?;

    Ok(())
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)] Io(#[cause] io::Error),
    #[fail(display = "`out_dir` is not valid UTF-8")] OutDir,
    #[fail(display = "`filband` is not valid UTF-8")] Filband,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
