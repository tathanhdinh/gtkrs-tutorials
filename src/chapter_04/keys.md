# Binding Keys

Finally, we are going to also program some events for when the user presses
certain keys and key combinations. In particular, to full screen the window
when the user presses **F11**, and to save the file when the user presses
**Ctrl+S**.

## Fullscreen State & App::connect_events() Changes

We are going to want to know when we should invoke either the
**Window::fullscreen()** or **Window::unfullscreen()** methods, and so it
will be important for us to store the state of the application's
fullscreen-ness within a new **AtomicBool** that will represent the current
state. This will be passed into our upcoming **App::key_events()** method,
which also will take a reference to the current file's **ActiveMetadata**
in order to perform operations like saving the active file.

```rust
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
        self.key_events(current_file, fullscreen);
    }

    // Wrap the `App` within `ConnectedApp` to enable the developer to execute the program.
    ConnectedApp(self)
}
```

## Implementing the App::key_events() Method

This is where the **gdk** crate immediately becomes important, as it will
help us to determine which keys have been pressed by the user, and whether
or not certain states were also enabled (such as whether Ctrl was held or
not).

Using the **connect_key_press_event()** method on the main window, we can
handle key press events that will be sent through the **gdk** variable
below. All you have to do is match the received state and key values, and
then execute the functionality that you want to achieve.

```rust
/// Handles special functions that should be invoked when certain keys and key combinations
/// are pressed on the keyboard.
fn key_events(
    &self,
    current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    fullscreen: Arc<AtomicBool>,
) {
    // Grab required references beforehand.
    let editor = self.content.source.buff.clone();
    let headerbar = self.header.container.clone();
    let save_button = self.header.save.clone();

    // Each key press will invoke this function.
    self.window.connect_key_press_event(move |window, gdk| {
        match gdk.get_keyval() {
            // Fullscreen the UI when F11 is pressed.
            key::F11 => if fullscreen.fetch_xor(true, Ordering::SeqCst) {
                window.unfullscreen();
            } else {
                window.fullscreen();
            },
            // Save the file when ctrl+s is pressed.
            key if key == 's' as u32 && gdk.get_state().contains(CONTROL_MASK) => {
                save(&editor, &headerbar, &save_button, &current_file, false);
            }
            _ => (),
        }
        Inhibit(false)
    });
}
```