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
