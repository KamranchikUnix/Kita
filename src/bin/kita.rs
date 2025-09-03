use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command as OsCommand, Stdio};

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
        /// Explicitly specify the C compiler to use (e.g., 'gcc', 'clang', 'cl.exe')
        #[arg(long, name = "c-compiler")]
        c_compiler: Option<String>,
    },
}

fn is_compiler_available(name: &str) -> bool {
    OsCommand::new(name)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn get_c_compiler_name(cli_arg: &Option<String>) -> String {
    if let Some(compiler) = cli_arg {
        return compiler.clone();
    }
    if let Ok(compiler) = env::var("KITA_CC") {
        return compiler;
    }

    if cfg!(target_os = "windows") {
        const WINDOWS_CANDIDATES: &[&str] = &["cl.exe", "clang.exe", "gcc.exe"];
        for &compiler in WINDOWS_CANDIDATES {
            if is_compiler_available(compiler) {
                return compiler.to_string();
            }
        }
        "cl.exe".to_string()
    } else {
        const UNIX_CANDIDATES: &[&str] = &["gcc", "clang"];
        for &compiler in UNIX_CANDIDATES {
            if is_compiler_available(compiler) {
                return compiler.to_string();
            }
        }
        "gcc".to_string()
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { file, output, save_c_source, c_compiler } => {
            build_file(file, output, save_c_source, &c_compiler)
        }
    }
}

fn build_file(
    path: PathBuf,
    output_path: Option<PathBuf>,
    save_c_source: bool,
    c_compiler_flag: &Option<String>,
) {
    println!("[1/4] Reading Source: {:?}", path);
    let source_code = fs::read_to_string(&path).expect("Failed to read file");

    println!("[2/4] Frontend Analysis...");
    let lexer = Lexer::new(source_code);
    let mut parser = KitaParser::new(lexer);
    let mut program = parser.parse_program();
    if !parser.errors.is_empty() {
        eprintln!("\nParsing failed with {} errors:", parser.errors.len());
        for err in parser.errors {
            eprintln!("\t- {}", err);
        }
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
    
    let output_file = output_path.unwrap_or_else(|| {
        let mut new_path = path.clone();
        new_path.set_extension("");
        if cfg!(target_os = "windows") {
            new_path.set_extension("exe");
        }
        new_path
    });

    let c_file_path = output_file.with_extension("c");
    fs::write(&c_file_path, &c_code).expect("Failed to write C source file");
    println!("     ...Generated C code at: {:?}", c_file_path);

    let compiler_name = get_c_compiler_name(c_compiler_flag);
    println!("[4/4] Compiling C code with: {}...", compiler_name);

    let mut command = OsCommand::new(&compiler_name);
    if compiler_name == "cl.exe" {
        command.arg("/nologo").arg("/O2").arg("/Fe:").arg(&output_file);
    } else {
        command.arg("-O2").arg("-o").arg(&output_file);
    }
    command.arg(&c_file_path);
    
    let status = command.status().expect(&format!("Failed to execute C compiler '{}'. Is it in your PATH?", compiler_name));
    
    if status.success() {
        println!("\n>>> Successfully built executable: {:?}", output_file);
        if !save_c_source {
            fs::remove_file(c_file_path).unwrap();
        }
    } else {
        eprintln!("\nC compilation failed. The intermediate C file was saved for debugging: {:?}", c_file_path);
    }
}
