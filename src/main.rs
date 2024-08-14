use hound::WavReader;
use serde::Serialize;
use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

#[derive(Serialize)]
struct SilentFiles {
    files: Vec<String>,
    bad_files: Vec<String>,
}

fn is_silent(file: &Path, threshold: f32) -> Result<bool, hound::Error> {
    let mut reader = WavReader::open(file)?;
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    let avg_abs: f32 = samples.iter().map(|&s| s.abs()).sum::<f32>() / samples.len() as f32;

    Ok(avg_abs <= threshold)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return;
    }

    let dir_path = &args[1];
    let threshold = 1e-6 as f32;
    let mut silent_files = SilentFiles {
        files: vec![],
        bad_files: vec![],
    };

    for entry in fs::read_dir(dir_path).expect("Directory not found") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
            match is_silent(&path, threshold) {
                Ok(true) => silent_files.files.push(path.to_string_lossy().to_string()),
                Ok(false) => (),
                Err(_) => silent_files
                    .bad_files
                    .push(path.to_string_lossy().to_string()),
            }
        }
    }

    let output_file = File::create("silent_files.json").expect("Failed to create output file");
    let writer = BufWriter::new(output_file);
    serde_json::to_writer_pretty(writer, &silent_files).expect("Failed to write JSON");
    println!("Silent files written to silent_files.json");
}
