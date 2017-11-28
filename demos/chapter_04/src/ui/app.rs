use super::{Content, Header, OpenDialog};
use super::misc::*;
use super::save::save;
use gdk::CONTROL_MASK;
use gdk::enums::key;
use gtk;
use gtk::*;
use preview::render;
use state::ActiveMetadata;
use std::fs::File;
use std::io::Read;
use std::process;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use webkit2gtk::*;

pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
}

/// A wrapped `App` which provides the capability to execute the program.
pub struct ConnectedApp(App);

impl ConnectedApp {
    /// Display the window, and execute the gtk main event loop.
    pub fn then_execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}

impl App {
    pub fn new() -> App {
        // Initialize GTK before proceeding.
        if gtk::init().is_err() {
            eprintln!("failed to initialize GTK Application");
            process::exit(1);
        }

        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        // Create a the headerbar and it's associated content.
        let header = Header::new();
        // Create the content container and all of it's widgets.
        let content = Content::new();

        // Set the headerbar as the title bar widget.
        window.set_titlebar(&header.container);
        // Set the title of the window.
        window.set_title("Markdown Editor");
        // Set the window manager class.
        window.set_wmclass("md-editor", "Markdown Editor");
        // The icon the app will display.
        window.set_default_size(800, 600);
        Window::set_default_icon_name("iconname");
        // Add the content to the window.
        window.add(&content.container);

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return the application structure.
        App { window, header, content }
    }

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
            self.key_events(current_file, fullscreen);
        }

        // Wrap the `App` within `ConnectedApp` to enable the developer to execute the program.
        ConnectedApp(self)
    }

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

    /// Programs the "Open" button for opening files.
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
}
