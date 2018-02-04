# File Chooser Dialogs

> Note that this is the dialogs.rs module

The GTK Rust API does not take advantage of the **Drop** trait within
Rust, which means that when a dialog is spawned, it will continue
to remain in existence on your screen forever. Thankfully, we can
address this ourselves by creating wrapper types for
**GtkFileChooserDialog**s and implementing the **Drop** trait on
them to destroy the inner dialog upon dropping the type.

## Creating an OpenDialog

With this, you will start by simply creating a new tuple type.

```rust
pub struct OpenDialog(FileChooserDialog);
```

And simply implementing the **new()** method on this type to
create a new inner **FileChooserDialog**.

```rust
impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> OpenDialog {
        // Create a new file chooser dialog for opening a file.
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        // Add the cancel and open buttons to that dialog.
        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        // Set the default path to open this with.
        path.map(|p| open_dialog.set_current_folder(p));

        OpenDialog(open_dialog)
    }
}
```

The gist of a **FileChooserDialog** is to designate the name of
the dialog, followed by providing a new window of the **Popup**
variety, and then choosing the corresponding **FileChooserAction**
that meets the criteria that you are going for. In this case, we
are creating an **Open** dialog to open a file.

Once created, you then need to specify the labels for the two
buttons within the dialog, and map the corresponding
**ResponseTypes** to those buttons. This is very important for
knowing whether the user hit **Cancel** or **Ok**.

Once we are all done, we simply need to wrap the type up in our
**OpenDialog** type and that's it.

## Creating a SaveDialog

The Save dialog is essentially identical.

```rust
pub struct SaveDialog(FileChooserDialog);

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> SaveDialog {
        // Initializes a new save as dialog, which is embedded within a new popup window.
        let save_dialog = FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        // Add the cancel and save buttons to that dialog.
        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        // Set the default path to open this with.
        path.map(|p| save_dialog.set_current_folder(p));

        SaveDialog(save_dialog)
    }
}
```

## Implementing the Drop Trait

Objects within GTK are destroyed with the **destroy()** method. So,
if we want to destroy our dialogs after we have thrown their handles
away, we can do this automatically by implementing the **Drop** trait
on both of the types by hand.

```rust
impl Drop for OpenDialog {
    fn drop(&mut self) { self.0.destroy(); }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}
```

Very stupid simple. We are simply calling the **destroy()** method
on the inner value of our new types.

## Implementing Helpful Methods

The following method can be added to the impl for both types, and they
simply make it easier to run and get the outputs that we want from the
dialogs.

```rust
pub fn run(&self) -> Option<PathBuf> {
    if self.0.run() == ResponseType::Ok.into() {
        self.0.get_filename()
    } else {
        None
    }
}
```

Basically, we display/run the dialog, and check the output to determine
whether or not we received the **Ok** response. If we received the **Ok**
response, we simply attempt to return the filename that may or may not
have been selected.

## The Final Code

```rust
use gtk::*;
use std::path::PathBuf;

/// A wrapped FileChooserDialog that automatically destroys itself upon being dropped.
pub struct OpenDialog(FileChooserDialog);

/// A wrapped FileChooserDialog that automatically destroys itself upon being dropped.
pub struct SaveDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new() -> OpenDialog {
        // Create a new file chooser dialog for opening a file.
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        // Add the cancel and open buttons to that dialog.
        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        OpenDialog(open_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl SaveDialog {
    pub fn new() -> SaveDialog {
        // Initializes a new save as dialog, which is embedded within a new popup window.
        let save_dialog = FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        // Add the cancel and save buttons to that dialog.
        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        SaveDialog(save_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) { self.0.destroy(); }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}
```
