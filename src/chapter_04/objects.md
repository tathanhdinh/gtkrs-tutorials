# File Chooser Dialogs, Source Views, & Web Views 

## GtkWebView

<img src="images/web_view.png" />

Provided by the [webkit2gtk crate](https://crates.io/crates/webkit2gtk), this a widget that
already integrates a scrolled window and simply provides an embedded web engine for rendering
HTML. The purpose of this widget within this chapter is to provide the rendered HTML
representations of the markdown text within the source buffer.

```rust
let context = WebContext::get_default().unwrap();
let webview = WebView::new_with_context(&context);
```

For our purposes, we will simply be shuttling the HTML output of the horrorshow crate into
the web view directly, which can be done by using the `load_html()` method.

```rust
webview.load_html(&html, None);
```

## GtkSourceView

<img src="images/source_view.png" />

Provided by the [sourceview crate](https://crates.io/crates/sourceview), this is an enhanced
**GtkTextView** which provides some much-needed code editing improvements. Alas, don't expect
too much from it, as it's still relatively primitive in it's current state.

```rust
let source_buffer = Buffer::new(None);
let source_view = View::new_with_buffer(&source_buffer);
```

The default settings are basically no different from a normal **GtkTextView** though, so
you will immediately want to configure the source view that you create according to your
needs, as we will see in this tutorial.

```rust
source_view.set_show_line_numbers(true);
source_view.set_monospace(true);
let font = FontDescription::from_string("monospace 11");
WidgetExt::override_font(&source_view, &font);
```

## GtkFileChooserDialog

<img src="images/file_chooser_dialog.png" />

**GtkFileChooserDialogs** will be used to program the file open/save/save as buttons. They
simply open a file chooser dialog where you are given the choice to select a file. The GTK
Rust API is a bit under-developed in this part, as you will see. One particular gotcha is
that the **FileChooserDialogs** that you obtain from this API do not take advantage of
Rust's **Drop** trait. Not to worry though, as we will be creating abstractions ourselves.

```rust
// Create a new file chooser dialog for opening a file.
let open_dialog = FileChooserDialog::new(
    Some("Open"),
    Some(&Window::new(WindowType::Popup)),
    FileChooserAction::Open,
);

// Add the cancel and open buttons to that dialog.
open_dialog.add_button("Cancel", ResponseType::Cancel.into());
open_dialog.add_button("Open", ResponseType::Ok.into());

// Run the dialog and the response type back.
if open_dialog.run() == ResponseType::Ok.into() {
    if let Some(filename) = open_dialog.get_filename() {
        // Do something with the received `PathBuf`.
    }
}

// Destroy the dialog window. Be careful not to return from a function
// without first destroying the dialog! A wrapper type will help.
open_dialog.destroy();
```
