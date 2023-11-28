# extern crate autd3_modulation_audio_file;
use autd3_modulation_audio_file::RawPCM;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let path = "path/to/foo.dat";
let m = RawPCM::new(&path, 4000)?;
# Ok(())
# }