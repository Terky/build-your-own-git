use anyhow::Result;
use flate2::bufread::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs::{create_dir, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};

pub fn hash_and_write_file(path: PathBuf) -> Result<String> {
    let source_file = File::open(path)?;
    let size = source_file.metadata()?.len();
    let mut reader = BufReader::new(source_file);

    let mut buffer = Vec::new();

    // Write the header
    buffer.extend("blob ".as_bytes());
    buffer.extend(size.to_string().as_bytes());
    buffer.push(0);

    // Write the content
    reader.read_to_end(&mut buffer)?;

    let hash = calculate_sha1(&mut buffer);

    let output_file = create_output_file(&hash)?;

    let mut zlib_reader = ZlibEncoder::new(BufReader::new(&buffer[..]), Compression::fast());

    std::io::copy(&mut zlib_reader, &mut BufWriter::new(output_file))?;

    Ok(hash)
}

fn calculate_sha1(buffer: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&buffer);
    hex::encode(hasher.finalize())
}

fn create_output_file(hash: &String) -> Result<File> {
    let sub_directory: String = hash.chars().take(2).collect();
    let file_name: String = hash.chars().skip(2).collect();
    let mut output_path = Path::new(".git").join("objects").join(sub_directory);
    if !output_path.exists() {
        create_dir(output_path.clone())?;
    }

    output_path = output_path.join(file_name);
    let file = File::create(output_path)?;
    Ok(file)
}
