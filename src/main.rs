use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

mod compilation_engine;
mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let source = env::args().nth(1).expect("usage: jack_analyzer <file.jack | directory>");
    let path = Path::new(&source);

    if path.is_file() {
        if !is_jack_file(path) {
            eprintln!("Error: input file must end with .jack");
            std::process::exit(1);
        }
        if let Err(e) = compile_one(path) {
            eprintln!("Failed to compile {}: {}", path.display(), e);
            std::process::exit(1);
        }
    } else if path.is_dir() {
        let mut jack_files: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(path).expect("failed to read directory") {
            let entry = entry.expect("bad directory entry");
            let p = entry.path();
            if p.is_file() && is_jack_file(&p) {
                jack_files.push(p);
            }
        }

        if jack_files.is_empty() {
            eprintln!("No .jack files found in directory {}", path.display());
            std::process::exit(1);
        }

        for jack in jack_files {
            if let Err(e) = compile_one(&jack) {
                eprintln!("Failed to compile {}: {}", jack.display(), e);
            }
        }
    } else {
        eprintln!("Error: path does not exist: {}", path.display());
        std::process::exit(1);
    }
}

fn is_jack_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("jack"))
        .unwrap_or(false)
}

fn output_xml_path(input_jack: &Path) -> PathBuf {
    let mut out = input_jack.to_path_buf();
    out.set_extension("xml");
    out
}

fn compile_one(input_jack: &Path) -> Result<(), String> {
    let out_path = output_xml_path(input_jack);
    let output = File::create(&out_path)
        .map_err(|e| format!("create {}: {}", out_path.display(), e))?;

    let input_str = input_jack
        .to_str()
        .ok_or_else(|| format!("non-utf8 path: {}", input_jack.display()))?;

    let mut tokenizer = Tokenizer::new(input_str);
    tokenizer.advance();

    let mut engine = compilation_engine::ComplationEngine::new(tokenizer, output);
    engine.comple_class();

    println!("Wrote {}", out_path.display());
    Ok(())
}
