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

        configure_source_view(&view, &buff);

        Source { container, buff, view }
    }
}

fn configure_source_view(view: &View, buff: &Buffer) {
    WidgetExt::override_font(view, &FontDescription::from_string("monospace"));

    LanguageManager::new()
        .get_language("markdown")
        .map(|markdown| buff.set_language(&markdown));

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("Builder")
        .or(manager.get_scheme("Classic"))
        .map(|theme| buff.set_style_scheme(&theme));

    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_insert_spaces_instead_of_tabs(true);
    view.set_indent_width(4);
    view.set_smart_backspace(true);
    view.set_right_margin(100);
    view.set_left_margin(10);
    view.set_show_right_margin(true);
    view.set_background_pattern(BackgroundPatternType::Grid);
    // TODO: Next GTK Crate Release
    // view.set_input_hints(InputHints::SPELLCHECK + InputHints::WORD_COMPLETION);
}