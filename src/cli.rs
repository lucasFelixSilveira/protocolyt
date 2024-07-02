#[derive(Debug, Clone)]
pub struct Cli {
  pub output: String,
  pub files: Vec<String>
}

impl Cli {
  pub fn init() -> Cli {
    Cli {
      output: String::new(),
      files: Vec::new()
    }
  }
}

pub fn parse_arguments(args: Vec<String>) -> Cli {
  let file_sufix: String = String::from(".ply");
  let mut st_cli: Cli = Cli::init();

  for arg in args {
    if arg.ends_with(&file_sufix) {
      st_cli.files.push(arg);
      continue;
    }
  }

  st_cli
}