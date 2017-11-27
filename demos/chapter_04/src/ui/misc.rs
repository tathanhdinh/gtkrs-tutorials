use gtk::*;
use sourceview::*;
use std::path::Path;

/// Sets the title of the header bar to the file name of the given path.
pub fn set_title(headerbar: &HeaderBar, path: &Path) {
    if let Some(filename) = path.file_name() {
        let filename: &str = &filename.to_string_lossy();
        headerbar.set_title(filename);
    }
}

/// Obtains the entire inner string of a given text buffer.
pub fn get_buffer(buffer: &Buffer) -> Option<String> {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true)
}
