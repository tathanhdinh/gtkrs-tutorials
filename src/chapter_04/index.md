# Simple Common Mark Editor

<img src="https://mmstick.github.io/gtkrs-tutorials/images/ch04_complete.png" />

In this chapter, we will create a useful program for editing text with
Markdown syntax, particularly the Common Mark flavor of Markdown, and
rendering that text into a live HTML preview as text is entered into
the program. We will also be using file chooser dialogs for opening
and saving files, and hashing the buffer to know when the save button
should be enabled. Once completed, you should have an understanding of
how to use not only the **gtk** crate, but **gdk**, **pango**,
**webkit2gtk**, and **sourceview** as well.

> This chapter was actually written with the completed program that
> we are going to build. So you could say that this tutorial is now
> fully self-hosted and is eating it's own dog food.

## Prerequisites

It's good to have an understanding of how to use types like **RwLock**
and **Mutex** before proceeding, as they will be utilized to maintain
external state of a custom type. They allow for otherwise immutable
values to be mutably borrowed across multiple threads and closures.

## Dependencies

Your dependency list should look similar to the following:

```toml
[dependencies]
gdk = "0.6.0"
horrorshow = "0.6.2"
pango = "0.2.0"
pulldown-cmark = "0.1.0"
sourceview = "0.2"
tiny-keccak = "1.4.0"
webkit2gtk = "0.2"

[dependencies.gtk]
features = ["v3_22"]
version = "0.2"
```

### pulldown-cmark

One will note that we have introduced a few new crates to the mix. Google
has provided the [pulldown-cmark crate](https://github.com/google/pulldown-cmark),
which provides a pull parser for Common Mark Markdown syntax, and thereby allows
us to convert Markdown into HTML.


### tiny-keccak

We will be using a hashing algorithm to determine when the save button should be
sensitive or not. Basically, we will hash the buffer upon each key press and compare
it to the saved file's hash. If the hashes match, the save button shall be enabled.
Otherwise, the save button will be disabled.s

### sourceview

This will provide access to the **GtkSourceView** widget that we will be using
to create the code editor that our editor will be using for editing Markdown.

### webkit2gtk

This will provide the **GtkWebView** widget which we will be using to render
a live preview of our edited Markdown.

### pango

This will enable us to edit the font in the **GtkSourceView**.

### gdk

We will use this to act upon certain key presses
