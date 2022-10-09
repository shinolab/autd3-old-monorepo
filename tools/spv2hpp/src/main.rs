use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
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
                ext.to_str().unwrap().contains("spv")
            } else {
                false
            }
        })
    {
        let f = File::open(e.path())?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        let path = e.path().parent().unwrap();
        let hppname = format!("{}.txt", e.file_name().to_str().unwrap());
        let path = path.join(hppname);

        let mut file = File::create(path)?;
        for b in buffer {
            write!(file, "{}, ", b)?;
        }
        file.flush()?;
    }

    Ok(())
}
