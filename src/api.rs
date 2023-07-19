use crate::error::{APIError, Error};

// use std::cmp::Ordering;
use std::path::Path;
use xmodits_lib::common::{extract, SUPPORTED_EXTENSIONS};
use xmodits_lib::exporter::AudioFormat;
use xmodits_lib::interface::ripper::Ripper;
// use xmodits_lib::interface::Error as XmoditsError;
use xmodits_lib::SampleNamer;

pub fn rip(
    path: &String,
    destination: String,
    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
    upper: Option<bool>,
    lower: Option<bool>,
    strict: Option<bool>,
    format: Option<String>,
) -> Result<(), Error> {
    let strict = strict.unwrap_or(true);
    verify_extension(path, strict).map_err(Error::from)?;

    let default_namer = SampleNamer::default();
    let ripper = Ripper::new(
        SampleNamer {
            index_only: index_only.unwrap_or_default(),
            index_padding: index_padding
                .map(|f| f as u8)
                .unwrap_or(default_namer.index_padding),
            index_raw: index_raw.unwrap_or_default(),
            lower: lower.unwrap_or_default(),
            upper: upper.unwrap_or_default(),
            ..default_namer
        }
        .into(),
        get_format(format)?.into(),
    );

    let self_contained = with_folder.unwrap_or_default();

    extract(path, &destination, &ripper, self_contained).map_err(Error::from)
}

pub fn verify_extension(path: &String, strict: bool) -> Result<(), APIError> {
    if !strict {
        return Ok(());
    }

    let ext = Path::new(path)
        .extension()
        .map(|f| f.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();

    if !SUPPORTED_EXTENSIONS.contains(&ext.as_ref()) {
        return Err(APIError::UnrecognizedFileExtension(ext));
    }

    Ok(())
}

fn get_format(format: Option<String>) -> Result<AudioFormat, APIError> {
    let Some(format) = format else {
        return Ok(AudioFormat::WAV);
    };

    let extension = format.to_lowercase();
    let format = match extension.as_str() {
        "wav" => AudioFormat::WAV,
        "aiff" => AudioFormat::AIFF,
        "8svx" => AudioFormat::IFF,
        "its" => AudioFormat::ITS,
        "s3i" => AudioFormat::S3I,
        "raw" => AudioFormat::RAW,
        _ => return Err(APIError::UnrecognizedFileExtension(extension)),
    };
    Ok(format)
}
