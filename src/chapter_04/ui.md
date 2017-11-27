# Creating the UI Structure

<img src="images/ch04_ui_diagram.png" />

Following the above diagram, we will begin by creating three modules
within the **ui** module: **app.rs**, **header.rs**, **content.rs**.
These three files will hold the **App**, **ConnectedApp**, **Header**,
**Content**, and **Source** structures.

## Implementing the UI Module

Now that UI-related code is separated into it's own module, it's
important to declare all of the submodules that this module will
important, and declare any types and functions that this module
should re-export.

```rust
mod app;
mod content;
mod dialogs;
mod header;
pub mod misc;
pub mod save;

pub use self::app::App;
pub use self::content::Content;
pub use self::dialogs::{OpenDialog, SaveDialog};
pub use self::header::Header;
```

As can be seen above, we are going to create the following modules:
**app.rs**, **content.rs**, **dialogs.rs**, **save.rs**, **header.rs**,
and **misc.rs**. 

## Implementing the App Structure

This section should be fairly straightforward, as it is essentially
identical to previous chapters. The one main difference to note
is that we have moved the initialization of GTK into the beginning
of the **App** implementation.

```rust
use gtk;
use gtk::*;
use super::Header;
use super::Content;

pub struct App {
    pub window:  Window,
    pub header:  Header,
    pub content: Content,
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
}
```

## Implementing the Header Structure


This structure will be even easier to implement. We will be implementing
**Open**, **Save**, and **Save As** buttons within the header bar. We
shall set their labels with a mnemonic, so that users have the ability
to hold the **Alt** key, and select the button that they want to click
by hitting the character that is underlined in the button label.

```rust
use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub open:      Button,
    pub save:      Button,
    pub save_as:   Button,
}

impl Header {
    pub fn new() -> Header {
        // Creates the main header bar container widget.
        let container = HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title("Markdown Editor");
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        let open = Button::new_with_mnemonic("_Open");
        let save = Button::new_with_mnemonic("_Save");
        let save_as = Button::new_with_mnemonic("Save _As");
        container.pack_start(&open);
        container.pack_end(&save_as);
        container.pack_end(&save);

        // Returns the header and all of it's state
        Header { container, open, save, save_as }
    }
}

```


## Implementing the Content Structure

This is where we are going to introduce **GtkWebViews**,
and **GtkSourceViews**. 

### Creating a Web View

Web views are very simple to create. By invoking the following code,
you will have constructed a web view that you may integrate within
your UI the same as you do with all other widgets.

```rust
// Create a the WebView for the preview pane.
let context = WebContext::get_default().unwrap();
let preview = WebView::new_with_context(&context);
```

These web views come already wrapped with scrolling capabilities,
so that functionality does not have to implemented.

### Creating & Configuring Source Views

Source views, on the other hand, are a bit more complex, because
they will require some additional configuration to get the results
that you want in your UI. We are going to strive to have the
following default configuration for our source view:

- Tabs should write four spaces
- The view should show line numbers
- We should use the default monospace font at size 11
- Markdown syntax highlighting
- Attempt to use the Builder theme by default, and fallback to Classic

First we start by constructing our **Source** structure.

```rust
pub struct Source {
    pub container: ScrolledWindow,
    pub view:      View,
    pub buff:      Buffer,
}

impl Source {
    fn new() -> Source {

    }
}
```

Then we will create our source buffer and view, which is identical to
how we would do it with a regular text buffer and view. Once created,
we will wrap the view within a scrolled window.

```rust
// Create the SourceView for the editor on the left pane.
let buff = Buffer::new(None);
let view = View::new_with_buffer(&buff);
let container = ScrolledWindow::new(None, None);
container.add(&view);
```

Then we set our desired configurations:

```rust
view.set_show_line_numbers(true);
view.set_monospace(true);
view.set_insert_spaces_instead_of_tabs(true);
view.set_indent_width(4);
// TODO: Next GTK Crate Release
// view.set_input_hints(InputHints::SPELLCHECK + InputHints::WORD_COMPLETION);
```

We can use the **pango** crate to change the font within the view. Note
that we needed to explicitly declare from which trait the **override_font**
method comes from. A future GTK Rust API update may address this issue.

```rust
// Configures the font to use with our source view, which shall be the default monospace
// font, at size 11. When overriding the font, we need to explicitly state the trait
// from where the method is coming from, due to two methods implementing the same method.
let font = FontDescription::from_string("monospace 11");
WidgetExt::override_font(&view, &font);
```

Then we want to enable markdown syntax highlighting by default. Languages
are obtained from a **LanguageManager**. We will assign the language
directly to the source buffer, rather than the source view.

```rust
// Configure markdown syntax highlighting
if let Some(markdown) = LanguageManager::new().get_language("markdown") {
    buff.set_language(&markdown);
}
```

### Completed Source Code

Altogether, the source code for this file should be as follows:

```rust
use gtk::*;
use pango::*;
use sourceview::*;
use webkit2gtk::*;

pub struct Content {
    pub container: Paned,
    pub source:    Source,
    pub preview:   WebView,
}

pub struct Source {
    pub container: ScrolledWindow,
    pub view:      View,
    pub buff:      Buffer,
}

impl Content {
    pub fn new() -> Content {
        // Create the Paned container for the main content
        let container = Paned::new(Orientation::Horizontal);
        let source = Source::new();

        // Create a the WebView for the preview pane.
        let context = WebContext::get_default().unwrap();
        let preview = WebView::new_with_context(&context);

        // Pack it in
        container.pack1(&source.container, true, true);
        container.pack2(&preview, true, true);

        // Ensure that the two panes get half the size of the paned container.
        source.container.set_size_request(100, -1);
        preview.set_size_request(100, -1);

        Content { container, source, preview }
    }
}

impl Source {
    pub fn new() -> Source {
        // Create the SourceView for the editor on the left pane.
        let buff = Buffer::new(None);
        let view = View::new_with_buffer(&buff);
        let container = ScrolledWindow::new(None, None);
        container.add(&view);

        // Set source view settings
        view.set_show_line_numbers(true);
        view.set_monospace(true);
        view.set_insert_spaces_instead_of_tabs(true);
        view.set_indent_width(4);

        // Configures the font to use with our source view, which shall be the default monospace
        // font, at size 11. When overriding the font, we need to explicitly state the trait
        // from where the method is coming from, due to two methods implementing the same method.
        let font = FontDescription::from_string("monospace 11");
        WidgetExt::override_font(&view, &font);

        // Configure markdown syntax highlighting
        if let Some(markdown) = LanguageManager::new().get_language("markdown") {
            buff.set_language(&markdown);
        }

        // Set the theme of source buffer
        let manager = StyleSchemeManager::new();
        if let Some(theme) = manager.get_scheme("Builder").or(manager.get_scheme("Classic")) {
            buff.set_style_scheme(&theme);
        }

        Source { container, buff, view }
    }
}
```