# Programming the Save Buttons

The **Save** and **Save As** buttons are a bit more complicated, in part due to
the need to set the sensitivity of the **Save** button over time, and that the
**Save** button should not open a dialog when there is an active file, but
immediately save that file to the disk.

## App::save_event()

We shall do something a bit different with the **Save** and **Save As** buttons.
We will declare a **App::save_event()** method we shall integrate within the
**App::connect_events()** method, but we will leave the implementation details
to a **save()** function within the **save.rs** module.

The key parameters that we need for the two dialogs is **save_button** that we
are going to program, the actual **Save** button which is named
**actual_button**, access to the current file's **ActiveMetadata**, and an
indication of whether the provided **save_button** is a **Save As** button
or not.

```rust
// Utilized for programming the "Save" and "Save As" buttons.
fn save_event(
    &self,
    save_button: &Button,
    actual_button: &Button,
    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    save_as: bool,
) {
    let editor = self.content.source.buff.clone();
    let headerbar = self.header.container.clone();
    let save_button = save_button.clone();
    actual_button.connect_clicked(
        move |_| save(&editor, &headerbar, &save_button, &current_file, save_as),
    );
}
```

## Updated App::connect_event()

In this next step, the **App::connect_event()** method should be written as so,
where we shall see that we have added two new method invocations to
**App::save_event()**. The first invocation will program the **Save** button,
whereas the second invocation will program the **Save As** button.

```rust
/// Creates external state, and maps all of the UI functionality to the UI.
pub fn connect_events(self) -> ConnectedApp {
    // External state to share across events.
    let current_file = Arc::new(RwLock::new(None));
    // Keep track of whether we are fullscreened or not.
    let fullscreen = Arc::new(AtomicBool::new(false));

    {
        let save = &self.header.save;
        let save_as = &self.header.save_as;

        // Connect all of the events that this UI will act upon.
        self.editor_changed(current_file.clone(), &save.clone());
        self.open_file(current_file.clone());
        self.save_event(&save.clone(), &save.clone(), current_file.clone(), false);
        self.save_event(&save, &save_as, current_file.clone(), true);
    }

    // Wrap the `App` within `ConnectedApp` to enable the developer to execute the program.
    ConnectedApp(self)
}
```

## Implementing the save.rs Module

First we will start with the imports that this module is going to require:

```rust
use super::SaveDialog;
use super::misc::*;
use gtk::*;
use sourceview::*;
use state::ActiveMetadata;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::RwLock;
```

Then we are going to want to know whether an action saved a new file, overwrote
the active file, or the save action was cancelled, and so we are going to create
an enum to make that distinction:

```rust
pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}
```

## Writing Data & Getting a SaveAction

The **write_data** private function will be used to write the given source
buffer's **data** to a file, and report back with the result of the save
action. If this was the **Save** button and there exists a pre-existing
**ActiveMetadata**, then this will merely write the data to the existing
file and return an **Ok(SaveAction::Saved)**.

Otherwise, if this was the **Save As** button that was clicked, or the **Save**
button was clicked when there wasn't an active metadata stored, then a
**SaveDialog** will be spawned to obtain the new file and return an
**Ok(SaveAction::New(ActiveMetadata))**. Unless, of course, the user
happened to cancel dialog, in which case we should return an
**Ok(SaveAction::Canceled)**.

```rust
/// Saves the given data to the file path supplied. If the path is **None**, a save dialog will
/// run to obtain the required path from the user. In the event that the dialog has to run to
/// obtain a file path, this function will return **Ok(Some(path))**, otherwise **Ok(None)**.
/// An **Err** value indicates an I/O-related error occurred when trying to save the file.
fn write_data(path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    if let Some(path) = path {
        // Save the given data to the given file, truncating the file beforehand.
        let mut file =
            OpenOptions::new().create(true).write(true).truncate(true).open(path.get_path())?;
        file.write_all(&data)?;
        return Ok(SaveAction::Saved);
    }
    
    let save_dialog = SaveDialog::new(None);
    if let Some(new_path) = save_dialog.run() {
        let mut file =
            OpenOptions::new().create(true).write(true).truncate(false).open(&new_path)?;
        file.write_all(data)?;
        Ok(SaveAction::New(ActiveMetadata::new(new_path, data)))
    } else {
        Ok(SaveAction::Canceled)
    }
}
```

### Writing the Public save() Function

Finally, we can get on to writing the public **save()** function within this
module. The first step will naturally be to obtain the text within the source
buffer for writing. Then we shall conditionally provide the current file's
metadata depending on whether this is the **Save As** button, or the **Save**
button. Finally, we will act upon the returned **SaveAction**, where a **New**
variant will provide the new metadata that we will store as the currently
active metadata, and to update the title and subtitle accordingly; and the
**Saved** action will signal that we need to get the sum for the text that
was written to the disk, and update the currently file's sum to reflect the
new state of the file.

```rust
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
            write_data(None, text.as_bytes())
        } else {
            write_data(current_file.read().unwrap().as_ref(), text.as_bytes())
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
```
