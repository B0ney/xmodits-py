use pyo3::exceptions::PyIOError;
use pyo3::PyErr;
use xmodits_lib::interface::Error as XmoditsError;

macro_rules! batch_create_exceptions {
    ($($EXCEPTION:ident) *) => {
        $(
            pyo3::create_exception!(xmodits, $EXCEPTION, pyo3::exceptions::PyException);
        )*
    };
}

batch_create_exceptions!(
    SampleExtraction
    TotalExtraction
    PartialExtraction
    UnsupportedFormat
    InvalidModule
    InvalidSample
    EmptyModule
    NoFormatFound
    AudioFormat
    MultipleErrors
    Generic
);

pub enum Error {
    Single(XmoditsError),
    Multiple(Vec<XmoditsError>),
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Single(inner) => match inner {
                XmoditsError::Io(e) => PyIOError::new_err(e.to_string()),
                XmoditsError::PartialExtraction(e) => PartialExtraction::new_err(partial(e)),
                XmoditsError::TotalExtraction(e) => TotalExtraction::new_err(total_failure(e)),
                XmoditsError::Extraction(e) => SampleExtraction::new_err(e),
                XmoditsError::UnsupportedModule(e) => UnsupportedFormat::new_err(e),
                XmoditsError::InvalidModule(e) => InvalidModule::new_err(e),
                XmoditsError::EmptyModule => EmptyModule::new_err(empty_module()),
                XmoditsError::AudioFormat(e) => AudioFormat::new_err(audio_format(e)),
                XmoditsError::BadSample { raw_index, .. } => {
                    InvalidSample::new_err(invalid_sample(raw_index))
                }
                XmoditsError::NoFormatFound => NoFormatFound::new_err(no_format_found()),
            },
            Error::Multiple(_) => MultipleErrors::new_err("Multiple Errors".to_string()), // todo
        }
    }
}

fn empty_module() -> String {
    "The module has no samples".into()
}

fn invalid_sample(index: u16) -> String {
    format!("Sample with raw index {index} points to an invalid offset")
}

fn no_format_found() -> String {
    "Could not determine a valid format".into()
}

fn audio_format(error: String) -> String {
    format!("Could not export sample to desired format: {error}")
}

fn partial(_: Vec<XmoditsError>) -> String {
    format!("Could not extract everything")
}

fn total_failure(_: Vec<XmoditsError>) -> String {
    format!("Could not extract anything, the module might be corrupted")
}
