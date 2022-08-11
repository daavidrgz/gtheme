use std::{
    fs::{self, DirEntry, File},
    io::{self, Write},
    path::{Path, PathBuf},
};
pub fn expand_path(origin_path: &str) -> PathBuf {
    let mut path = String::from(origin_path);
    path = shellexpand::tilde(&path).to_string();
    path = shellexpand::env(&path).unwrap().to_string();
    PathBuf::from(path)
}

pub fn read_dir<P>(path: P) -> io::Result<Vec<DirEntry>>
where
    P: AsRef<Path>,
{
    let files = fs::read_dir(path.as_ref())?
        .into_iter()
        .filter_map(|dir| dir.ok())
        .collect();
    Ok(files)
}

pub fn write_to<P>(content: &str, path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if let Some(prefix) = path.as_ref().parent() {
        fs::create_dir_all(prefix)?;
    }
    let mut output_file = File::create(path)?;
    output_file.write_all(content.as_bytes())
}

pub fn copy_dir_all<P>(src: P, dst: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
