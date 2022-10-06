use anyhow::Result;
use image::GenericImageView;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];

    for e in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                ext.to_str().unwrap().contains("png")
            } else {
                false
            }
        })
    {
        let img = image::open(e.path()).expect("File not found!");

        let path = e.path().parent().unwrap();
        let hppname = format!("{}.txt", e.file_name().to_str().unwrap());
        let path = path.join(hppname);

        let mut file = File::create(path)?;
        for (_, _, rgba) in img.pixels() {
            write!(file, "{}, ", rgba.0[0])?;
            write!(file, "{}, ", rgba.0[1])?;
            write!(file, "{}, ", rgba.0[2])?;
            write!(file, "{}, ", rgba.0[3])?;
        }
        file.flush()?;
    }

    Ok(())
}
