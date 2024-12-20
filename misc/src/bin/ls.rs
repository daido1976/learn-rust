use std::env;
use std::fs;
use std::io::Write;
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

fn list_directory<W: Write>(writer: &mut W, path: &Path) -> std::io::Result<()> {
    let output = fs::read_dir(path)?
        .map(|entry| {
            let entry = entry?;
            Ok(FilePresenter::new(
                entry.file_name().to_string_lossy().into_owned(),
                entry.path().is_dir(),
            )
            .to_pretty())
        })
        .collect::<std::io::Result<Vec<_>>>()?
        .join("  ");
    writeln!(writer, "{}", output)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir().unwrap();
    let target_path = args.get(1).map_or(current_dir.as_path(), |p| Path::new(p));
    list_directory(&mut std::io::stdout(), target_path).expect("Failed to list directory");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_list_directory() {
        let tempdir = tempdir().unwrap();
        let dir_path = tempdir.path();

        fs::write(dir_path.join("testfile.txt"), "content").unwrap();
        fs::create_dir(dir_path.join("testdir")).unwrap();

        let mut buffer = Vec::new();
        list_directory(&mut buffer, dir_path).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "\u{1b}[34mtestdir\u{1b}[0m  testfile.txt\n");
    }
}
