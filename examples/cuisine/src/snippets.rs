use granola::{macros::*, prelude::*};

const SVG_RECT: &str = "data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%3E%3Crect%20width='96'%20height='96'%20fill='peachpuff'/%3E%3C/svg%3E";

pub fn snippets() -> HtmlSection {
    section!(
        hr!(),
        headings(),
        paragraphs(),
        blockquote(),
        lists(),
        figure(),
        table(),
        form_controls(),
    )
}

fn headings() -> HtmlDiv {
    div!(h1!("Heading 1"), h2!("Heading 2"), h3!("Heading 3"))
}

fn paragraphs() -> HtmlDiv {
    let p1 = p![
        "A paragraph with a ",
        a!("hyperlink").href("#select-reset"),
        "."
    ];
    let p2 = p![
        "Another paragraph with a ",
        strong!("strong"),
        ", ",
        b!("bold"),
        ", ",
        code!("code"),
        ", ",
        small!("small"),
        ", ",
        sub!("sub"),
        ", ",
        sup!("sup"),
        ", and an ",
        abbr!("abbr").title("abbreviation"),
        " elements.",
    ];

    div!(p1, p2)
}

fn lists() -> HtmlDiv {
    let unordered = ul![li!("First item"), li!("Second item")];

    let ordered = ol![li!("First item"), li!("Second item")];

    div!(unordered, ordered)
}

fn blockquote() -> HtmlBlockquote {
    blockquote!(p!("A block quotation."))
}

fn figure() -> HtmlFigure {
    let img = img!()
        .src(SVG_RECT)
        .alt("A peach puff colored square")
        .width(96)
        .height(96);

    figure!(figcaption!("Fig caption"), img)
}

fn table() -> HtmlTable {
    let caption = caption!("Table caption");

    let head = thead!(tr!(
        th!("Header 1").scope("col"),
        th!("Header 2").scope("col"),
    ));

    let body = tbody!(
        tr!(td!("Cell 1"), td!("Cell 2")),
        tr!(td!("Cell 3"), td!("Cell 4")),
    );

    table!(caption, head, body)
}

fn form_controls() -> HtmlFieldset {
    fieldset!(
        legend!("Form controls"),
        div!(
            label!("Textarea ").for_id("snippets-notes"),
            textarea!()
                .placeholder("Textarea")
                .id("snippets-notes")
                .rows(3),
        ),
        div!(
            label!("Text input ").for_id("snippets-text"),
            input!()
                .id("snippets-text")
                .input_type(InputType::Text)
                .placeholder("Text"),
        ),
        div!(
            label!("Search input ").for_id("snippets-search"),
            input!()
                .id("snippets-search")
                .input_type(InputType::Search)
                .placeholder("Search"),
        ),
        button!("Button").button_type(ButtonType::Button),
    )
}
