use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};
use rfd::{MessageButtons, MessageDialog, FileDialog};

fn main() -> io::Result<()> {
    // 1. Define predefined path (temporary directory + installer.zip)
    let mut predefined_path = std::env::temp_dir();
    predefined_path.push("installer.zip");

    // 2. Show initial message
    MessageDialog::new()
        .set_title("Folder Selection")
        .set_description("Choose the folder containing the files to package")
        .set_buttons(MessageButtons::Ok)
        .show();

    // 3. Folder selection dialog
    let folder = FileDialog::new()
        .set_title("Source Folder")
        .pick_folder()
        .unwrap_or_else(|| {
            MessageDialog::new()
                .set_title("Error")
                .set_description("No folder was selected")
                .set_buttons(MessageButtons::Ok)
                .set_level(rfd::MessageLevel::Error)
                .show();
            std::process::exit(1);
        });

    // 4. Create ZIP directly at the predefined path
    if let Err(e) = crear_zip(&predefined_path, &folder) {
        MessageDialog::new()
            .set_title("Error")
            .set_description(&format!("Error creating ZIP: {}", e))
            .set_buttons(MessageButtons::Ok)
            .set_level(rfd::MessageLevel::Error)
            .show();
        std::process::exit(1);
    }

    // 5. Show confirmation with the used path
    MessageDialog::new()
        .set_title("Success")
        .set_description(&format!("ZIP created at:\n{}", predefined_path.display()))
        .set_buttons(MessageButtons::Ok)
        .show();

    Ok(())
}

/// Recursively traverses `root_dir` and adds each file to the ZIP archive at `zip_path`.
fn crear_zip(zip_path: &Path, root_dir: &Path) -> io::Result<()> {
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
