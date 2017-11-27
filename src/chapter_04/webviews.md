# Updating WebViews

The first capability that we will connect to our UI is to update the web view
preview dynamically as text in the source buffer is changed. We will begin
by creating a new **editor_changed()** method for **App**, which takes the
**current_file** component, and a reference to the **save** button so that we
can disable and enable the button over time, to indicate whether changes have
been made that require saving.

```rust
/// Updates the WebView when the SourceBuffer is modified.
fn editor_changed(
    &self,
    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    save_button: &Button,
) {
    let preview = self.content.preview.clone();
    let save_button = save_button.clone();
    self.content.source.buff.connect_changed(move |editor| {
        if let Some(markdown) = get_buffer(&editor) {
            preview.load_html(&render(&markdown), None);
            if let Some(ref current_file) = *current_file.read().unwrap() {
                let has_same_sum = current_file.is_same_as(&markdown.as_bytes());
                save_button.set_sensitive(!has_same_sum);
            }
        }
    });
}
```

## Connect Changed

In the above code, you may notice that we are calling utilizing the
**connect_changed** method from the source buffer to update the preview
and modify the save button as the buffer is modified.

## Obtaining Text From a Source Buffer

And if you recall from a previous chapter, we are getting the text within
a buffer using a useful abstraction:

```rust
/// Obtains the entire inner string of a given text buffer.
pub fn get_buffer(buffer: &Buffer) -> Option<String> {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true)
}

```

```rust
if let Some(markdown) = get_buffer(&editor) {

}
```

Which obtains the text within the editor

## Updating the Web View

```rust
preview.load_html(&render(&markdown), None);
```

Whereas here we are using the **render()** function from the **preview** module
to convert the `markdown` text into an HTML string; and then immediately pass
that HTML into our `preview` web view with the **load_html()** method.

## Modifying the Save Button

The last piece of the puzzle is this section, where we obtain a read-only
lock to the current file, and check if the text in the buffer generates
the same hash as the the hash we saved to the disk. If there's a match,
the save button will be disabled; and if there's no match, it will be
enabled.

```rust
if let Some(ref current_file) = *current_file.read().unwrap() {
    if current_file.is_same_as(&markdown.as_bytes()) {
        save_button.set_sensitive(false);
    } else {
        save_button.set_sensitive(true);
    }
}
```