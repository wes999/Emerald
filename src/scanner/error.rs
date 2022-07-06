use ariadne::{Report, ReportKind, Label, Source};

pub enum Error {
    UnrecognizedToken,
    TypeMismatch,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Error::UnrecognizedToken => String::from("Unrecognized Token"),
            Error::TypeMismatch => String::from("Mismatched Types"),
        }
    }

    pub fn report_error(&self, character: usize) {
        Report::build(ReportKind::Error, (), 34)
            .with_message(self.to_string())
            .with_label(Label::new(32..33).with_message("This is of type Nat"))
            .with_label(Label::new(42..45).with_message("This is of type Str"))
            .finish();
    }
}