use std::path::Path;
use xmodits_lib::common::extract;
use xmodits_lib::interface::ripper::Ripper;
use xmodits_lib::interface::Error;
use xmodits_lib::SampleNamer;

pub fn rip_multiple<'a>(
    paths: &[String],
    destination: String,
    index_raw: Option<bool>,
    index_padding: Option<usize>,
    index_only: Option<bool>,
    with_folder: Option<bool>,
    upper: Option<bool>,
    lower: Option<bool>,
) -> Result<(), Error> {
    let mut ripper = Ripper::default();
    let default_namer = SampleNamer::default();
    let index_padding = index_padding
        .map(|f| f as u8)
        .unwrap_or(default_namer.index_padding);

    ripper.change_namer(
        SampleNamer {
            index_only: index_only.unwrap_or_default(),
            index_padding,
            index_raw: index_raw.unwrap_or_default(),
            lower: lower.unwrap_or_default(),
            upper: upper.unwrap_or_default(),
            ..default_namer
        }
        .into(),
    );

    let self_contained = with_folder.unwrap_or_default();

    // Collect errors during dumping
    let mut errors: Vec<Error> = paths
        .into_iter()
        .filter(|path| Path::new(path).is_file())
        .map(|path| extract(path, destination, &ripper, self_contained))
        .filter_map(|result| result.err())
        .collect();

    use std::cmp::Ordering;
    // Compare size of errors
    // return Ok(()) if errors.len() = 0
    // Extract a single error & return it if errors.len() = 1
    // Construct "MultipleErrors" to contain errors and return it if errors.len() > 1

    match errors.len().cmp(&1) {
        Ordering::Less => Ok(()),
        Ordering::Equal => Err(errors.pop().unwrap()),
        Ordering::Greater => todo!(),
    }
}

fn into(a: Option<bool>) -> bool {
    a.unwrap_or_default()
}
