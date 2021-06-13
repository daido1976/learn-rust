use nix::sys::wait::*;
use nix::unistd::*;
use std::ffi::CString;
use std::io;
use std::path::*;
use std::str::FromStr;
use Status::Success;

fn main() {
    let lsh_loop = LshLoop::new();
    match lsh_loop.start() {
        Ok(_) => {}
        Err(err) => println!("{}", err.message),
    }
}

struct LshLoop;

impl LshLoop {
    fn new() -> LshLoop {
        LshLoop
    }

    fn start(&self) -> Result<Status, LshError> {
        loop {
            let mut s = String::new();
            let scan = io::stdin();
            let _ = scan.read_line(&mut s);
            let args: Vec<String> = s.split_whitespace().map(String::from).collect();

            match execute(args) {
                Ok(status) => match status {
                    Success => continue,
                    exit @ Status::Exit => return Ok(exit),
                },
                err @ Err(_) => return err,
            };
        }
    }
}

enum Commands {
    Cd,
    Help,
    Exit,
    Execute,
}

impl FromStr for Commands {
    type Err = LshError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "cd" {
            Ok(Commands::Cd)
        } else if s == "help" {
            Ok(Commands::Help)
        } else if s == "exit" {
            Ok(Commands::Exit)
        } else {
            Ok(Commands::Execute)
        }
    }
}

/// Execute shell build-in or launch program.
fn execute(args: Vec<String>) -> Result<Status, LshError> {
    if args.is_empty() {
        return Err(LshError::new("Empty command"));
    }

    Commands::from_str(&args[0]).and_then(|c: Commands| match c {
        Commands::Cd => lsh_cd(&args[1]),
        Commands::Help => Ok(lsh_help()),
        Commands::Exit => Ok(lsh_exit()),
        Commands::Execute => lsh_launch(args),
    })
}

fn lsh_cd(dir: &str) -> Result<Status, LshError> {
    if dir.is_empty() {
        Err(LshError::new("lsh: expected argument to cd\n"))
    } else {
        chdir(Path::new(&dir))
            .map(|_| Success)
            .map_err(|err| LshError::new(&err.to_string()))
    }
}

fn lsh_help() -> Status {
    println!("daido1976's LSH inspired by lsh");
    println!("Type program names and arguments, and hit enter.");
    println!("The following are build in:");
    println!("Use the man command for information on other programs.");
    Success
}

fn lsh_exit() -> Status {
    Status::Exit
}

fn lsh_launch(args: Vec<String>) -> Result<Status, LshError> {
    let pid;
    unsafe {
        pid = fork().map_err(|_| LshError::new("fork failed"))?;
    }
    match pid {
        ForkResult::Parent { child } => {
            let wait_pid_result =
                waitpid(child, None).map_err(|err| LshError::new(&format!("{}", err)));
            match wait_pid_result {
                Ok(WaitStatus::Exited(_, _)) => Ok(Success),
                Ok(WaitStatus::Signaled(_, _, _)) => Ok(Success),
                Err(err) => Err(LshError::new(&err.message)),
                _ => Ok(Success),
            }
        }
        ForkResult::Child => {
            let path = CString::new(args[0].to_string()).unwrap();
            let args = if args.len() > 1 {
                CString::new(args[1].to_string()).unwrap()
            } else {
                CString::new("").unwrap()
            };
            execv(&path, &[path.clone(), args])
                .map(|_| Success)
                .map_err(|_| LshError::new("Child Process failed"))
        }
    }
}
struct LshError {
    message: String,
}

impl LshError {
    fn new(message: &str) -> LshError {
        LshError {
            message: message.to_string(),
        }
    }
}

enum Status {
    Success,
    Exit,
}
