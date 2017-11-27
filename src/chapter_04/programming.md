# Connecting Events

This is where we begin to do the magic of getting our UI to actually
do stuff. Revisiting our **app.rs** module, we will now add the
**connect_events()** module to the impl for **App**. Yet before we begin,
we should create the **ConnectedApp** wrapper in advance, which implements
the **then_execute()** method that we saw before.

```rust
/// A wrapped `App` which provides the capability to execute the program.
pub struct ConnectedApp(App);

impl ConnectedApp {
    /// Display the window, and execute the gtk main event loop.
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}
```

Moving on...

```rust
/// Creates external state, and maps all of the UI functionality to the UI.
pub fn connect_events(self) -> ConnectedApp {
    // External state to share across events.
    let current_file = Arc::new(RwLock::new(None));

    // Connect all of the events that this UI will act upon.
    self.editor_changed(current_file.clone(), &self.header.save.clone());

    // Wrap the `App` within `ConnectedApp` to enable the developer to execute the program.
    ConnectedApp(self)
}
```

As seen above, we are starting out with a single external component, which
we have named as **current_file**. This is what will store an
**Option<ActiveMetadata>**, the type which we created for keeping track of
what file is currently opened, and a hash that will be used to indicate
whether the save button should be sensitive or not. A button that is not
sensitive, is a button that you cannot click.

The next section will detail that **editor_changed()** method.