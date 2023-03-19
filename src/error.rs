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
    Generic
);

pub struct Error(pub XmoditsError);

impl Error {
    pub fn py_err(error: XmoditsError) -> PyErr {
        Self(error).into()
    }
}

impl From<Error> for PyErr {
    fn from(Error(inner): Error) -> Self {
        match inner {
            XmoditsError::Io(e) => PyIOError::new_err(e.to_string()),
            XmoditsError::PartialExtraction(e) => partial(e),
            XmoditsError::TotalExtraction(e) => total_failure(e),
            XmoditsError::Extraction(e) => SampleExtraction::new_err(e),
            XmoditsError::UnsupportedModule(e) => UnsupportedFormat::new_err(e),
            XmoditsError::InvalidModule(e) => InvalidModule::new_err(e),
            XmoditsError::EmptyModule => empty_module(),
            XmoditsError::AudioFormat(e) => audio_format(e),
            XmoditsError::BadSample { raw_index, .. } => invalid_sample(raw_index),
            XmoditsError::NoFormatFound => no_format_found(),
        }
    }
}

fn empty_module() -> PyErr {
    let err: String = "The module has no samples".into();
    EmptyModule::new_err(err)
}

fn invalid_sample(index: u16) -> PyErr {
    let err: String = format!("Sample with raw index {index} points to an invalid offset");
    InvalidSample::new_err(err)
}

fn no_format_found() -> PyErr {
    let err: String = format!("Could not determine a valid format");
    InvalidSample::new_err(err)
}

fn audio_format(error: String) -> PyErr {
    let err: String = format!("Could not export sample to desired format: {error}");
    AudioFormat::new_err(err)
}

fn partial(_: Vec<XmoditsError>) -> PyErr {
    let err: String = format!("Could not extract everything");
    AudioFormat::new_err(err)
}

fn total_failure(_: Vec<XmoditsError>) -> PyErr {
    let err: String = format!("Could not extract anything, the module might be corrupted");
    AudioFormat::new_err(err)
}
