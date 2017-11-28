# The ui/misc.rs Module

We are going to implement a few helper methods that will be utilized across
the project on an as-needed basis. This is purely to serve some useful
abstractions. The two functions that we are going to provide is an abstraction
for obtaining the text within a **GtkSourceBuffer**, and for setting the title
of a **GtkHeaderBar** with a given **Path**.

```rust
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
```