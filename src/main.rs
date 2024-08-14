use hound::{WavReader};
use serde::Serialize;
use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::io::BufWriter;

#[derive(Serialize)]
struct SilentFiles {
    files: Vec<String>,
}

// fn is_silent(file_path: &Path, threshold: f64) -> bool {
//     let mut reader = WavReader::open(file_path).expect("Failed to open WAV file");
//     reader.samples::<i16>()
//         .all(|sample| sample.unwrap().abs() as f64 <= threshold)
// }

fn is_silent(file: &Path, threshold: f32) -> bool {
    let mut reader = WavReader::open(file).expect("Failed to open WAV file");
    let samples: Vec<f32> = reader.samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();
    
    let avg_abs: f32 = samples.iter().map(|&s| s.abs()).sum::<f32>() / samples.len() as f32;
    
    avg_abs <= threshold
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return;
    }

    let dir_path = &args[1];
    let mut silent_files = SilentFiles { files: vec![] };

    for entry in fs::read_dir(dir_path).expect("Directory not found") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
            if is_silent(&path, 1e-4) {
                silent_files.files.push(path.to_string_lossy().to_string());
            }
        }
    }

    let output_file = File::create("silent_files.json").expect("Failed to create output file");
    let writer = BufWriter::new(output_file);
    serde_json::to_writer_pretty(writer, &silent_files).expect("Failed to write JSON");
    println!("Silent files written to silent_files.json");
}
