use nom_exif::*;

pub async fn test_exif() -> anyhow::Result<()> {
    println!("hi");
    let mut parser = MediaParser::new();

    let files = [
        "/mnt/nand/scratch/renders/A001_01051049_C040.mov",
        "/home/dopey/braw-test/A001_12032234_C002.braw",
        "/mnt/storage/tove/immich/auto-transcoded/A001_01031837_C018.mov",
    ];

    for f in files {
        println!("{:?}", f);
        let ms = MediaSource::file_path(f)?;
        println!("{:?}", ms);
        if ms.has_exif() {
            // Parse the file as an Exif-compatible file
            println!("has exif");
            let mut iter: ExifIter = parser.parse(ms)?;
            // ...
        } else if ms.has_track() {
            println!("has track");
            // Parse the file as a track
            let info: TrackInfo = parser.parse(ms)?;
            for entry in info {
                println!("{:?}", entry);
            }
            //println!("{:?}", info);
            //println!("{:?}", info.get(TrackInfoTag::CreateDate));
            //println!("{:?}", info.get(TrackInfoTag::DurationMs));
            // ...
        }
    }

    Ok(())
}
