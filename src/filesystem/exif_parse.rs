use crate::lib_models::ExifMetaData;
use chrono::{DateTime, FixedOffset, Local, Offset};
use nom_exif::*;

pub async fn parse_exif(file: String) -> Result<(ExifMetaData)> {
    let mut parser = MediaParser::new();
    println!("After parser");
    let ms = MediaSource::file_path(file).inspect_err(handle_parsing_error)?;
    println!("After ms");

    let metadata = if ms.has_track() {
        let info: TrackInfo = parser.parse(ms)?;
        println!("info from track {:?}", info);
        let creation_date = info
            .get(TrackInfoTag::CreateDate)
            .and_then(EntryValue::as_time);

        let duration_ms = info
            .get(TrackInfoTag::DurationMs)
            .and_then(EntryValue::as_u64)
            .map(|d| d as i32); // Convert u64 to i32 if needed

        println!("creation date from exif {:?}", creation_date);
        ExifMetaData {
            creation_date,
            duration_ms,
        }
    } else {
        let now: DateTime<Local> = Local::now();
        let now_fixed_offset: DateTime<FixedOffset> = now.with_timezone(&now.offset().fix());
        println!("The time being used {:?}", now_fixed_offset);

        ExifMetaData {
            creation_date: Option::from(now_fixed_offset),
            duration_ms: None,
        }
    };

    //metadata.iter().for_each(|x| {
    //    println!("{:<32}=> {}", x.0, x.1);
    //});
    println!("{:?}", metadata);
    Ok(metadata)
}

fn handle_parsing_error(e: &nom_exif::Error) {
    match e {
        nom_exif::Error::UnrecognizedFileFormat => {
            eprintln!("Unrecognized file format, consider filing a bug @ https://github.com/mindeng/nom-exif.");
        }
        nom_exif::Error::ParseFailed(_) | nom_exif::Error::IOError(_) => {
            eprintln!("Error: {e}");
        }
    }
}
