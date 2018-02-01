use std::fmt;
use failure::Fail;

#[derive(Fail, Debug)]
pub struct ErrorList<T: Fail> {
    pub errs: Vec<T>,
}

impl<T: Fail> fmt::Display for ErrorList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.errs
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
