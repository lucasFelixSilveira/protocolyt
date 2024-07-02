use std::{env, fs, path};
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use which::which;
use colored::*;

use lexer::utils::Bytes;

mod cli;
mod lexer;
mod error;
mod parser;

fn main() {
    if which("bun").is_err() {
        println!("We need to make some preparations. Wait.");

        if cfg!(target_os = "windows") {
            let status = Command::new("powershell")
                .arg("-c")
                .arg("irm bun.sh/install.ps1 | iex")
                .status()
                .expect("Failed to execute PowerShell command");

            if !status.success() {
                println!("{}", "Failed to install required packages.".red());
                return;
            }

            Command::new("powshell").arg("clear");
        } else {
            let status = Command::new("sh")
                .arg("-c")
                .arg("curl -fsSL https://bun.sh/install | bash")
                .status()
                .expect("Failed to execute sh command");

            if !status.success() {
                println!("{}", "Failed to install required packages.".red());
                return;
            }

            Command::new("sh").arg("clear");
        }

        println!("The packages ask you to restart your PC.");
        std::thread::sleep(std::time::Duration::new(10, 0));
    }

    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);

    let r_cli: cli::Cli = cli::parse_arguments(arguments);

    let current_path: String = env::current_dir().unwrap().display().to_string();
    let slash: char = path::MAIN_SEPARATOR;
    let mut files: Vec<(String, String)> = Vec::new();

    if r_cli.files.len() == 0 {
        println!("Please, inform a main file.");
        return;
    }

    

    for file in r_cli.files {
        let path_: String = format!("{current_path}{slash}{file}");
        let mut file_content: (Bytes, String, String) = match fs::read_to_string(&path_) {
            Ok(text) => (Bytes::from(text.clone()), text, path_),
            Err(_) => {
                error::spawn(error::Error::IOe, "An invalid file name was sent to standard input.".to_string(), None).unwrap();
                (Bytes::null(), String::new(), path_)
            }
        };

        let tokens: Vec<lexer::Token> = lexer::parse(&mut file_content);
        files.push(
            (
                parser::parse(&mut file_content, tokens), 
                file
            )
        );
    }

    let first_name: String = files[0].clone().1.rsplit_once("/").unwrap().1.to_string();
    let first_no_extension: &str = first_name.rsplit_once('.').unwrap().0;
    let exec_path: String = env::current_exe().unwrap().display().to_string().rsplit_once(slash).unwrap().0.to_string();
    let std_path: String = format!("{exec_path}{slash}{first_no_extension}");
    if fs::read_dir(&std_path).is_err() {
        fs::create_dir(&std_path).expect("fail to create file");
    }

    let mut main_path: String = String::new();
    for out_file in files {
        let name: String = out_file.1.rsplit_once("/").unwrap().1.to_string();
        let no_extension: &str = name.rsplit_once('.').unwrap().0;
        let _path: String = format!("{std_path}{slash}{}{slash}{}", r_cli.output, [no_extension, ".ts"].concat());
        fs::write(_path.clone(), out_file.0).unwrap();
        if main_path.is_empty() {
            main_path = _path;
        }
    }

    let mut child = Command::new("bun")
        .arg("run")
        .arg(main_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start script");

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(err) => eprintln!("Error reading stdout: {}", err),
            }
        }
    });

    let stderr = child.stderr.take().expect("Failed to open stderr");
    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => eprintln!("{}", line),
                Err(err) => eprintln!("Error reading stderr: {}", err),
            }
        }
    });

    let status = child.wait().expect("Failed to wait on child");
    stdout_thread.join().expect("Failed to join stdout thread");
    stderr_thread.join().expect("Failed to join stderr thread");

    eprintln!("{}: {}", if status.success() { "Exit status code".bright_green() } else { "Exit status code".red() }, format!("{}", status).rsplit_once(' ').unwrap().1);

}  
