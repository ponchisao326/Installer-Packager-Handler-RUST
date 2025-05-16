use std::{fs::File, io, path::Path};
use std::io::{Read, Write};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

#[unsafe(no_mangle)]
pub extern "C" fn create_zip(folder_path: *const libc::c_char, zip_path: *const libc::c_char) -> libc::c_int {
    let folder = unsafe { std::ffi::CStr::from_ptr(folder_path).to_string_lossy().into_owned() };
    let zip = unsafe { std::ffi::CStr::from_ptr(zip_path).to_string_lossy().into_owned() };

    match crear_zip(Path::new(&zip), Path::new(&folder)) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}

/// Recursively traverses `root_dir` and adds each file to the ZIP archive at `zip_path`.
pub fn crear_zip(zip_path: &Path, root_dir: &Path) -> io::Result<()> {
    let file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::<()>::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);

    for entry in walkdir::WalkDir::new(root_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let rel_path = path.strip_prefix(root_dir).unwrap();
        let name = rel_path.to_string_lossy();

        let mut f = File::open(path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        zip.start_file(name.as_ref(), options)?;
        zip.write_all(&buffer)?;
    }

    zip.finish()?;
    Ok(())
}
