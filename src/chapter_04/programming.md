# Connecting Events

This is where we begin to do the magic of getting our UI to actually
do stuff. Revisiting our **app.rs** module, we will now add the
**App::connect_events()** method.

## ConnectedApp

Before we begin, however, we should create the **ConnectedApp** wrapper
in advance, which implements the **then_execute()** method that we saw before.
The goal will be to convert the **App** into a **ConnectedApp** after the
**App::connect_events()** method is invoked on the **App**.

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

## connect_events()

Moving on, we will finally add the **App::connect_events()** method to the **impl**
for the **App** type, which we will define to take ownership of the **App**
and return a **ConnectedApp** at the end. This is 

```rust
/// Creates external state, and maps all of the UI functionality to the UI.
impl App {
    pub fn new() -> App { ... }

    pub fn connect_events(self) -> ConnectedApp {
        // External state to share across events.
        let current_file = Arc::new(RwLock::new(None));

        // Connect all of the events that this UI will act upon.
        self.editor_changed(
            current_file.clone(),
            &self.header.save.clone()
        );

        // Wrap the `App` within `ConnectedApp` to enable the developer
        // to execute the program.
        ConnectedApp(self)
    }
}
```

Using the **ActiveMetadata** type that we created before to maintain the
extern state in regards to the currently-active file, we will create a 
**RwLock'd** **current_file** variable, which will contain an
**Option\<ActiveMetadata\>**. By default, this will be set to **None**,
as at first there will not be a file opened to track.

The first event to connect to our application will be to act upon changes
to the source buffer, which we will implement in a **App::editor_changed()**
method. This method will take a reference to our **RwLock'd** **current_file**,
as well as a reference to the **Save** button. The purpose of passing the
save button in will be to modify it's *sensitivity* based on the contents of
the buffer.

> A button that is not sensitive, is a button that you cannot click.