# Setting Up Modules

<img src="images/ch04_mod_diagram.png" />

With this chapter, we are going to begin to use modules to compartmentalize
code within manageable files. Following the above diagram, we shall separate
the GTK-specific code from the rest of the codebase accordingly.

- The **ui** module will contain all of the code relevant to building and
executing the GTK program.
- The **preview** module will be responsible for generating the HTML from the
provided Markdown.
- The **state** module will be responsible for holding our custom types that
will be managing external state within the program.

## Creating Modules

<img src="images/ch04_mod_structure.png" />

The **preview** and **state** modules are relatively simple, and so they
can simply be individual Rust files: **preview.rs** and **state.rs**. The
**ui** module will be much more complex, however, and so that module will
consist of a series of modules on it's own. As a result, it should be a
**ui** directory where the point of entry into this module will be a
**mod.rs** file within that directory.

## The main.rs File

Once we have the basic modules set up, we will wire everything together
to the main point of entry into our program: the **main.rs** file. It
should look like so to get started.

```rust
extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate horrorshow;
extern crate pango;
extern crate pulldown_cmark;
extern crate sourceview;
extern crate tiny_keccak;
extern crate webkit2gtk;

pub mod preview;
pub mod state;
pub mod ui;

use ui::App;

fn main() {
    // Initialize the UI's initial state
    App::new()
        // Connect events to the UI
        .connect_events()
        // Display the UI and execute the program
        .then_execute();
}
```

You may notice that we have changed things up from the last chapter. In
this chapter, we are going to use the **Builder** pattern to set up and
execute our program. This can be a useful pattern to employ that can
eliminate would-be API logic errors at compile time.

The `App::new()` method will create a new **App**, which we will move
into the `connect_events()` method, which converts the **App** into a
**ConnectedApp**, and that type implements `then_execute()`, which shall
display the UI and execute the main event loop.