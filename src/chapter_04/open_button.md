# Programming the Open Button

Now it is time to use those dialogs to program the **Open**, **Save**, and
**Save As** buttons in our UI. We shall start by first implementing the
**Open** button by creating a new method called **open_file()**, which we
shall integrate within our **connect_events()** method.

## Open Button

The **open_file()** button will obtain access to our **current_file**
entity, which will be updated upon successfully opening the selected
file.

```rust
pub fn connect_events(self) -> ConnectedApp {
    // External state to share across events.
    let current_file = Arc::new(RwLock::new(None));

    // Connect all of the events that this UI will act upon.
    self.editor_changed(current_file.clone(), &self.header.save.clone());
    self.open_file(current_file.clone());

    // Wrap the `App` within `ConnectedApp` to enable the developer to execute the program.
    ConnectedApp(self)
}
```

### connect_clicked()

Now, the **open_file** method will simply be grabbing references to the editor's
source buffer, to write the opened file's data into the buffer; then grabbing a
reference to the web view so that we can update it after opening the file; a
reference to the header bar so that we can update the title and subtitle; and
finally the **Open** button itself, so that we can map the **connect_clicked()**
event to it.

```rust
fn open_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
    let editor = self.content.source.buff.clone();
    let preview = self.content.preview.clone();
    let headerbar = self.header.container.clone();
    self.header.open.connect_clicked(move |_| {
        // Program the button here
    });
}
```

### Creating an OpenDialog

Within **connect_clicked()**, we shall start by creating a new **OpenDialog**.
When opening this dialog, we are to attempt to pass in the **current_file**'s
parent directory, if it exists, so that the open file dialog uses that
directory in the file search view by default.

> Note that I am using **if let Some(ref path)** rather than a **map**
> here due to borrowing concerns -- if you can't get a map to borrow correctly,
> using a **match** or **if let** usually does the trick.

```rust
// Create a new open file dialog using the current file's parent
// directory as the preferred directory, if it's set.
let open_dialog = OpenDialog::new({
    let lock = current_file.read().unwrap();
    if let Some(ref path) = *lock {
        path.get_dir()
    } else {
        None
    }
});
```

### Running the Dialog

Once we have our **open_dialog** variable, it's then just a matter of running
the dialog, grabbing the selected file path, and then handling that file
accordingly: read the file into the source buffer, update the web view,
and then update the title and subtitle.

```rust
// Runs the dialog, and opens the file if a file was selected.
if let Some(new_file) = open_dialog.run() {
    if let Ok(mut file) = File::open(&new_file) {
        // Read the file's contents into an in-memory buffer
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        // Update the title and subtitle
        set_title(&headerbar, &new_file);
        if let Some(parent) = new_file.parent() {
            let subtitle: &str = &parent.to_string_lossy();
            headerbar.set_subtitle(subtitle);
        }

        // Set the shared file path as this file.
        *current_file.write().unwrap() =
            Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

        // Update the editor & preview
        editor.set_text(&contents);
        preview.load_html(&render(&contents), None);
    }
}
```

### The Completed Code

```rust
fn open_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
    let editor = self.content.source.buff.clone();
    let preview = self.content.preview.clone();
    let headerbar = self.header.container.clone();
    self.header.open.connect_clicked(move |_| {
        // Create a new open file dialog using the current file's parent
        // directory as the preferred directory, if it's set.
        let open_dialog = OpenDialog::new({
            let lock = current_file.read().unwrap();
            if let Some(ref path) = *lock {
                path.get_dir()
            } else {
                None
            }
        });

        // Runs the dialog, and opens the file if a file was selected.
        if let Some(new_file) = open_dialog.run() {
            if let Ok(mut file) = File::open(&new_file) {
                // Read the file's contents into an in-memory buffer
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);

                // Update the title and subtitle
                set_title(&headerbar, &new_file);
                if let Some(parent) = new_file.parent() {
                    let subtitle: &str = &parent.to_string_lossy();
                    headerbar.set_subtitle(subtitle);
                }

                // Set the shared file path as this file.
                *current_file.write().unwrap() =
                    Some(ActiveMetadata::new(new_file, &contents.as_bytes()));

                // Update the editor & preview
                editor.set_text(&contents);
                preview.load_html(&render(&contents), None);
            }
        }
    });
}
```