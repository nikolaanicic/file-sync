use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn walkdir(path: &Path, is_base: bool) -> HashSet<String> {
    log::debug!("walking {}", path.display());

    let entries = std::fs::read_dir(&path).unwrap();
    let mut paths_with_prefix = HashSet::new();

    for entry in entries {
        if let Ok(entry_path) = entry {
            if entry_path.file_type().unwrap().is_dir() {
                paths_with_prefix.extend(walkdir(&entry_path.path(), false));
            } else {
                paths_with_prefix.insert(entry_path.path().to_string_lossy().to_string());
            }
        }
    }

    if is_base {
        let mut result = HashSet::new();
        for p in paths_with_prefix {
            result.insert(
                Path::new(&p)
                    .strip_prefix(path)
                    .expect(format!("can't remove the prefix {}", path.display()).as_str())
                    .to_string_lossy()
                    .to_string(),
            );
        }

        return result;
    }

    return paths_with_prefix;
}

pub fn cmp_dirs(source: &Path, destination: &Path) -> std::io::Result<HashMap<String, bool>> {
    log::debug!(
        "comparing {} to {}",
        source.display(),
        destination.display()
    );

    let source_files = walkdir(source, true);
    let destination_files = walkdir(destination, true);

    let mut result: HashMap<String, bool> = HashMap::new();

    for path in source_files.iter() {
        let exists = destination_files.contains(path);
        result.insert(path.to_owned(), exists);
    }

    Ok(result)
}

fn clear_dir(dir_path: &Path) {
    log::debug!(
        "removing and recreating the directory: {}",
        dir_path.display()
    );

    _ = fs::remove_dir_all(dir_path);
    _ = fs::create_dir(dir_path);
}

pub fn sync_dir(source: &Path, destination: &Path) -> std::io::Result<()> {
    log::debug!("syncing {} to {}", source.display(), destination.display());

    clear_dir(&destination);
    let filenames = std::fs::read_dir(source)?;

    for file_path in filenames {
        if let Ok(file_path) = file_path {
            let new_path = destination.join(&file_path.file_name());
            if file_path.file_type().unwrap().is_dir() {
                log::debug!("found directory {}", new_path.display());
                fs::create_dir(&new_path)?;
                sync_dir(&source.join(&file_path.file_name()), &new_path)?;
            } else {
                log::debug!("creating a file {}", new_path.display());
                fs::File::create(&new_path)?;
                fs::copy(&source.join(file_path.file_name()), &new_path)?;
            }
        }
    }

    Ok(())
}
