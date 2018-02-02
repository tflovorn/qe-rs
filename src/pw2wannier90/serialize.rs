use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::File;
use serialize_util::push_bool_field;
use pw2wannier90::input::Input;

pub fn make_input_file(input: &Input) -> Result<String, Error> {
    let mut lines = Vec::new();
    lines.push(String::from(" &inputpp"));

    lines.push(format!("   prefix='{}',", input.prefix));

    if let Some(ref out_dir) = input.out_dir {
        let path = out_dir.to_str().ok_or(Error::OutDir)?;
        lines.push(format!("   out_dir='{}',", path));
    }

    lines.push(format!("   seedname='{}',", input.seedname));

    push_bool_field(&mut lines, "write_unk", Some(input.write_unk));
    push_bool_field(&mut lines, "write_amn", Some(input.write_amn));
    push_bool_field(&mut lines, "write_mmn", Some(input.write_mmn));
    push_bool_field(&mut lines, "write_spn", Some(input.write_spn));

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
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
