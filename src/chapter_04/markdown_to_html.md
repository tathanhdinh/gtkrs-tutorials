# Markdown to HTML

Before we implement the **connect_changed()** method, we need to first
implement the **preview** module that this method will use to get the
HTML string to pass on to the web view.

There are two steps to the process of converting Markdown into HTML. The first
step involves simply converting the Markdown into HTML! Yet that's not enough,
you also need to embed that within additional HTML to style it and get some
syntax highlighting support with a bit of JavaScript. Not to worry though,
because we will be using **highlight.js** to take care of syntax highlighting
for us.

## Converting Markdown to HTML

Google has thankfully provided a crate to do just this with
[pulldown-cmark](https://github.com/google/pulldown-cmark). It notes that
it is implemented as a pull parser for increased efficiency compared to
non-pull parsers. All you have to do is provide Markdown text in a **&str**
to the provider **Parser**, and then suppliy a mutable **String** reference
to pull in the HTML equivalent of the markup.

```rust
use pulldown_cmark::{html, Parser};

/// In goes Markdown text; out comes HTML text.
fn mark_to_html(markdown: &str) -> String {
    let parser = Parser::new(&markdown);
    let mut buffer = String::new();
    html::push_html(&mut buffer, parser);
    buffer
}
```

## Applying Styling to Our HTML

But we don't want to stop there, so we will use the above function within
our public **render()** function to integrate it alongside some CSS and
JavaScript to get the desired HTML output in the web view.

> Note that we are supplying the HTML from our Markdown into the **body**
> section of the HTML page, and have it wrapped as a **Raw** string to
> tell the **horrorshow** macro to not escape the inner text. You may
> apply additional styling if you would prefer even more styling to your
> text.

```rust
use horrorshow::Raw;
use horrorshow::helper::doctype;

/// In goes Markdown text; out comes stlyish HTML text.
pub fn render(markdown: &str) -> String {
    format!(
        "{}",
        html!(
            : doctype::HTML;
            html {
                head {
                    link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/github.min.css") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                    script {
                        : Raw("hljs.initHighlightingOnLoad()")
                    }
                    style {
                        : "body { width: 80%; margin: 0 auto }";
                        : "img { max-width: 80% }"
                    }
                }
                body {
                    : Raw(&mark_to_html(markdown));
                }
            }
        )
    )
}
```
