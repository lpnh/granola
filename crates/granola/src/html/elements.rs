pub mod edits;
pub use edits::*;
pub mod embedded;
pub use embedded::*;
pub mod forms;
pub use forms::*;
pub mod interactive;
pub use interactive::*;
pub mod metadata;
pub use metadata::*;
pub mod multimedia;
pub use multimedia::*;
// pub mod root;
// pub use root::*;
// pub mod scripting;
// pub use scripting::*;
// pub mod sections;
// pub use sections::*;
// pub mod tabular;
// pub use tabular::*;
pub mod text_content;
pub use text_content::*;
pub mod text_semantics;
pub use text_semantics::*;
// pub mod web_components;
// pub use web_components::*;

// https://developer.mozilla.org/en-US/docs/Web/HTML/Guides/Content_categories#phrasing_content
#[allow(dead_code)]
enum PhrasingElement {
    Abbr,
    Audio,
    B,
    Bdi,
    Bdo,
    Br,
    Button,
    Canvas,
    Cite,
    Code,
    Data,
    Datalist,
    Dfn,
    Em,
    Embed,
    I,
    Iframe,
    Img,
    Input,
    Kbd,
    Label,
    Mark,
    Math,
    Meter,
    Noscript,
    Object,
    Output,
    Picture,
    Progress,
    Q,
    Ruby,
    S,
    Samp,
    Script,
    Select,
    Slot,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Svg,
    Template,
    Textarea,
    Time,
    U,
    Var,
    Video,
    Wbr,
    // === Conditionally phrasing ===
    //
    // only if it contains only phrasing content:
    A,
    Del,
    Ins,
    Map,
    // only if it is a descendant of a <map> element:
    Area,
    // only if the `itemprop` attribute is present:
    Link,
    Meta,
}
