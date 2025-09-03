use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::Command as OsCommand;

use kita_bin::backend::codegen_c::CTranspiler;
use kita_bin::frontend::{lexer::Lexer, parser::Parser as KitaParser, sema::SemanticAnalyzer};

#[derive(Parser, Debug)]
#[command(version, author, about = "The Kita Programming Language Compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Builds a .ki file into a native executable by transpiling to C
    Build {
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Keep the intermediate C source file for debugging
        #[arg(short, long, name = "save-c")]
        save_c_source: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { file, output, save_c_source } => build_file(file, output, save_c_source),
    }
}

fn build_file(path: PathBuf, output_path: Option<PathBuf>, save_c_source: bool) {
    println!("[1/4] Reading Source: {:?}", path);
    let source_code = fs::read_to_string(&path).expect("Failed to read file");

    println!("[2/4] Frontend Analysis...");
    let mut lexer = Lexer::new(source_code);
    let mut parser = KitaParser::new(lexer);
    let mut program = parser.parse_program();

    if !parser.errors.is_empty() {
        eprintln!("\nParsing failed with {} errors:", parser.errors.len());
        for err in parser.errors { eprintln!("\t- {}", err); }
        return;
    }

    let mut sema = SemanticAnalyzer::new();
    if let Err(err) = sema.analyze(&mut program) {
        eprintln!("\nSemantic analysis failed: {}", err);
        return;
    }
    println!("     ...Frontend analysis successful.");


    println!("[3/4] Backend C Transpilation...");
    let mut transpiler = CTranspiler::new();
    let c_code = transpiler.transpile(program).expect("Failed to transpile to C");

    let output_file = output_path.unwrap_or_else(|| path.with_extension(""));
    let c_file_path = output_file.with_extension("c");
    fs::write(&c_file_path, &c_code).expect("Failed to write C source file");
    println!("     ...Generated C code at: {:?}", c_file_path);


    println!("[4/4] Compiling C code with MSVC (cl.exe)...");
    let status = OsCommand::new("cl.exe")
        .arg(&c_file_path)
        .arg("/nologo")      // Suppress startup banner
        .arg("/O2")          // Optimize for speed
        .arg("/Fe:")         // /Fe specifies the output executable name
        .arg(&output_file)
        .status()
        .expect("Failed to execute C compiler (cl.exe). Are you in a Developer Command Prompt for VS?");
    
    if status.success() {
        println!("\n>>> Successfully built executable: {:?}", output_file);
        if !save_c_source {
            fs::remove_file(c_file_path).unwrap();
        }
    } else {
        eprintln!("\nC compilation failed. The intermediate C file was saved for debugging: {:?}", c_file_path);
    }
}
