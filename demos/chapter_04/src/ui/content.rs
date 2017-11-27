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
