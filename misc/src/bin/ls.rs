use std::env;
use std::fs;
use std::path::Path;

struct FilePresenter {
    name: String,
    is_dir: bool,
}

impl FilePresenter {
    fn new(name: String, is_dir: bool) -> Self {
        Self { name, is_dir }
    }

    fn to_pretty(&self) -> String {
        if self.is_dir {
            // Blue color
            format!("\x1b[34m{}\x1b[0m", &self.name)
        } else {
            self.name.clone()
        }
    }
}

fn get_files_from_directory(path: &Path) -> Vec<FilePresenter> {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            FilePresenter::new(
                entry.file_name().to_string_lossy().into_owned(),
                entry.path().is_dir(),
            )
        })
        .collect()
}

fn convert_files_to_output(files: &[FilePresenter]) -> String {
    files
        .iter()
        .map(|x| x.to_pretty())
        .collect::<Vec<_>>()
        .join("  ")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir().unwrap();
    let target_path = args.get(1).map_or(current_dir.as_path(), |p| Path::new(p));

    let files = get_files_from_directory(target_path);
    let output = convert_files_to_output(&files);
    println!("{}", output);
}
