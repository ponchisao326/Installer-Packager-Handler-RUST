use std::{
    io::{self, Read, Write},
};
use rfd::{MessageButtons, MessageDialog, FileDialog};
mod lib;
use lib::crear_zip;


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


