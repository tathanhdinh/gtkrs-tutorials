use super::SaveDialog;
use super::misc::*;
use gtk::*;
use sourceview::*;
use state::ActiveMetadata;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::RwLock;

pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}

pub fn save(
    editor: &Buffer,
    headerbar: &HeaderBar,
    save: &Button,
    current_file: &RwLock<Option<ActiveMetadata>>,
    save_as: bool,
) {
    if let Some(text) = get_buffer(editor) {
        // If we are programming the "Save As" button then we will not use the
        // current path. Otherwise, we will save the editor's text to the
        // current path, if there is a current path.
        let result = if save_as {
            _save(None, text.as_bytes())
        } else {
            _save(current_file.read().unwrap().as_ref(), text.as_bytes())
        };

        // Now we are to match the result of the save function's output. We should
        // only act upon a return that returns an **Ok(Some(ActiveMetadata))**, setting
        // the title of the header bar, and the current file to the new path that
        // we have received.
        match result {
            Ok(SaveAction::New(file)) => {
                // Update the title and subtitle
                set_title(&headerbar, file.get_path());
                if let Some(parent) = file.get_dir() {
                    let subtitle: &str = &parent.to_string_lossy();
                    headerbar.set_subtitle(subtitle);
                }
                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
            }
            Ok(SaveAction::Saved) => {
                if let Some(ref mut current_file) = *current_file.write().unwrap() {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
            }
            _ => (),
        }
    }
}

/// Saves the given data to the file path supplied. If the path is **None**, a save dialog will
/// run to obtain the required path from the user. In the event that the dialog has to run to
/// obtain a file path, this function will return **Ok(Some(path))**, otherwise **Ok(None)**.
/// An **Err** value indicates an I/O-related error occurred when trying to save the file.
fn _save(path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    match path {
        Some(path) => {
            // Save the given data to the given file, truncating the file beforehand.
            let mut file =
                OpenOptions::new().create(true).write(true).truncate(true).open(path.get_path())?;
            file.write_all(&data)?;
        }
        None => {
            let save_dialog = SaveDialog::new(None);
            if let Some(new_path) = save_dialog.run() {
                let mut file =
                    OpenOptions::new().create(true).write(true).truncate(false).open(&new_path)?;
                file.write_all(data)?;
                return Ok(SaveAction::New(ActiveMetadata::new(new_path, data)));
            } else {
                return Ok(SaveAction::Canceled);
            }
        }
    }

    Ok(SaveAction::Saved)
}
