use askama::Template;
use indexmap::IndexMap;

use crate::{filters, prelude::*};

/// The HTML global attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let img = HtmlImg::new()
///     .src("ship.png")
///     .id("cruiser")
///     .class("spaceship");
///
/// assert_eq!(
///     img.bake(),
///     r#"<img class="spaceship" id="cruiser" src="ship.png" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- accesskey | bake_attr("accesskey") -}}
/// {{- autocapitalize | bake_attr("autocapitalize") -}}
/// {{- class | bake_attr("class") -}}
/// {{- contenteditable | bake_attr("contenteditable") -}}
/// {{- dir | bake_attr("dir") -}}
/// {{- draggable | bake_attr("draggable") -}}
/// {{- enterkeyhint | bake_attr("enterkeyhint") -}}
/// {{- exportparts | bake_attr("exportparts") -}}
/// {{- hidden | bake_attr("hidden") -}}
/// {{- id | bake_attr("id") -}}
/// {{- inputmode | bake_attr("inputmode") -}}
/// {{- is | bake_attr("is") -}}
/// {{- itemid | bake_attr("itemid") -}}
/// {{- itemprop | bake_attr("itemprop") -}}
/// {{- itemref | bake_attr("itemref") -}}
/// {{- itemtype | bake_attr("itemtype") -}}
/// {{- lang | bake_attr("lang") -}}
/// {{- nonce | bake_attr("nonce") -}}
/// {{- part | bake_attr("part") -}}
/// {{- popover | bake_attr("popover") -}}
/// {{- role | bake_attr("role") -}}
/// {{- slot | bake_attr("slot") -}}
/// {{- spellcheck | bake_attr("spellcheck") -}}
/// {{- style | bake_attr("style") -}}
/// {{- tabindex | bake_attr("tabindex") -}}
/// {{- title | bake_attr("title") -}}
/// {{- translate | bake_attr("translate") -}}
/// {{- writingsuggestions | bake_attr("writingsuggestions") -}}
/// {{- autofocus | bake_bool_attr("autofocus") -}}
/// {{- inert | bake_bool_attr("inert") -}}
/// {{- itemscope | bake_bool_attr("itemscope") -}}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct GlobalAttrs {
    pub accesskey: Option<Bake>,
    pub autocapitalize: Option<Bake>,
    pub autocorrect: Option<Bake>,
    pub class: Option<Bake>,
    pub contenteditable: Option<Bake>,
    pub dir: Option<Bake>,
    pub draggable: Option<Bake>,
    pub enterkeyhint: Option<Bake>,
    pub exportparts: Option<Bake>,
    pub hidden: Option<Bake>,
    pub id: Option<Bake>,
    pub inputmode: Option<Bake>,
    pub is: Option<Bake>,
    pub itemid: Option<Bake>,
    pub itemprop: Option<Bake>,
    pub itemref: Option<Bake>,
    pub itemtype: Option<Bake>,
    pub lang: Option<Bake>,
    pub nonce: Option<Bake>,
    pub part: Option<Bake>,
    pub popover: Option<Bake>,
    pub role: Option<Bake>,
    pub slot: Option<Bake>,
    pub spellcheck: Option<Bake>,
    pub style: Option<Bake>,
    pub tabindex: Option<i64>,
    pub title: Option<Bake>,
    pub translate: Option<Bake>,
    pub writingsuggestions: Option<Bake>,
    pub autofocus: bool,
    pub inert: bool,
    pub itemscope: bool,
}

impl HasGlobalAttrs for GlobalAttrs {
    fn global_attrs_mut(&mut self) -> &mut GlobalAttrs {
        self
    }
}

impl HasGlobalAttrs for &mut GlobalAttrs {
    fn global_attrs_mut(&mut self) -> &mut GlobalAttrs {
        self
    }
}

pub trait HasGlobalAttrs: Sized {
    fn global_attrs_mut(&mut self) -> &mut GlobalAttrs;

    /// Keyboard shortcut to activate or focus element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/accesskey)
    fn accesskey(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().accesskey = Some(value.into());
        self
    }

    // NOTE: Include `anchor` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/anchor)

    /// Recommended autocapitalization behavior (for supported input methods).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/autocapitalize)
    fn autocapitalize(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().autocapitalize = Some(value.into());
        self
    }

    /// Recommended autocorrection behavior (for supported input methods).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/autocorrect)
    fn autocorrect(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().autocorrect = Some(value.into());
        self
    }

    /// Automatically focus the element when the page is loaded.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/autofocus)
    fn autofocus(mut self) -> Self {
        self.global_attrs_mut().autofocus = true;
        self
    }

    /// Classes to which the element belongs.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/class)
    fn class(mut self, value: impl Into<Bake>) -> Self {
        let new = value.into();
        let ga = self.global_attrs_mut();
        ga.class = Some(match ga.class.take() {
            None => new,
            Some(existing) => bake_ws![existing, new],
        });
        self
    }

    /// Whether the element is editable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/contenteditable)
    fn contenteditable(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().contenteditable = Some(value.into());
        self
    }

    /// The text directionality of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/dir)
    fn dir(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().dir = Some(value.into());
        self
    }

    /// Whether the element is draggable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/draggable)
    fn draggable(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().draggable = Some(value.into());
        self
    }

    /// Hint for selecting an enter key action.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/enterkeyhint)
    fn enterkeyhint(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().enterkeyhint = Some(value.into());
        self
    }

    /// Selects and styles elements in nested shadow trees by exporting their
    /// part names.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/exportparts)
    fn exportparts(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().exportparts = Some(value.into());
        self
    }

    /// Whether the element is relevant.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/hidden)
    fn hidden(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().hidden = Some(value.into());
        self
    }

    /// The element's ID.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/id)
    fn id(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().id = Some(value.into());
        self
    }

    /// Whether the element is inert.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inert)
    fn inert(mut self, value: bool) -> Self {
        self.global_attrs_mut().inert = value;
        self
    }

    /// Hint for selecting an input modality.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inputmode)
    fn inputmode(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().inputmode = Some(value.into());
        self
    }

    /// Creates a customized built-in element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/is)
    fn is(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().is = Some(value.into());
        self
    }

    /// Global identifier for a microdata item.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemid)
    fn itemid(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().itemid = Some(value.into());
        self
    }

    /// Property names of a microdata item.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemprop)
    fn itemprop(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().itemprop = Some(value.into());
        self
    }

    /// Referenced elements.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemref)
    fn itemref(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().itemref = Some(value.into());
        self
    }

    /// Introduces a microdata item.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemscope)
    fn itemscope(mut self, value: bool) -> Self {
        self.global_attrs_mut().itemscope = value;
        self
    }

    /// Item types of a microdata item.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemtype)
    fn itemtype(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().itemtype = Some(value.into());
        self
    }

    /// Language of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/lang)
    fn lang(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().lang = Some(value.into());
        self
    }

    /// Cryptographic nonce used in Content Security Policy checks.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/nonce)
    fn nonce(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().nonce = Some(value.into());
        self
    }

    /// List of the part names of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/part)
    fn part(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().part = Some(value.into());
        self
    }

    /// Makes the element a popover element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/popover)
    fn popover(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().popover = Some(value.into());
        self
    }

    /// WAI-ARIA role.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles)
    fn role(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().role = Some(value.into());
        self
    }

    /// The element's desired slot.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/slot)
    fn slot(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().slot = Some(value.into());
        self
    }

    /// Whether the element is to have its spelling and grammar checked.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/spellcheck)
    fn spellcheck(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().spellcheck = Some(value.into());
        self
    }

    /// Presentational and formatting instructions.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/style)
    fn style(mut self, value: impl Into<Bake>) -> Self {
        let new = value.into();
        let ga = self.global_attrs_mut();
        ga.style = Some(match ga.style.take() {
            None => new,
            Some(existing) => bake_ws![existing, new],
        });
        self
    }

    /// Whether the element is focusable and sequentially focusable, and the
    /// relative order of the element for the purposes of sequential focus
    /// navigation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/tabindex)
    fn tabindex(mut self, value: i64) -> Self {
        self.global_attrs_mut().tabindex = Some(value);
        self
    }

    /// Advisory information for the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/title)
    fn title(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().title = Some(value.into());
        self
    }

    /// Whether the element is to be translated when the page is localized.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/translate)
    fn translate(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().translate = Some(value.into());
        self
    }

    // NOTE: Include `virtualkeyboardpolicy` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/virtualkeyboardpolicy)

    /// Whether the element can offer writing suggestions or not.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/writingsuggestions)
    fn writingsuggestions(mut self, value: impl Into<Bake>) -> Self {
        self.global_attrs_mut().writingsuggestions = Some(value.into());
        self
    }
}

/// HTML Custom data (`data-*` attributes).
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/data-*)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let img = HtmlImg::from_src("ship.png")
///     .custom_data("ship-id", "1337")
///     .custom_data("weapons", "laserI laserII")
///     .custom_data("shields", "72%")
///     .custom_data("x", "414354")
///     .custom_data("y", "85160")
///     .custom_data("z", "31940");
///
/// assert_eq!(
///     img.bake_pretty(),
///     r#"<img
///   src="ship.png"
///   data-ship-id="1337"
///   data-weapons="laserI laserII"
///   data-shields="72%"
///   data-x="414354"
///   data-y="85160"
///   data-z="31940"
/// />
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- for (attr, value) in map.iter() %} data-{{ attr }}="{{ value }}"{% endfor -%}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CustomDataAttrs {
    map: IndexMap<Bake, Bake>,
}

impl HasCustomDataAttrs for &mut CustomDataAttrs {
    fn custom_data_attrs_mut(&mut self) -> &mut CustomDataAttrs {
        self
    }
}

pub trait HasCustomDataAttrs: Sized {
    fn custom_data_attrs_mut(&mut self) -> &mut CustomDataAttrs;

    fn custom_data(mut self, attr: impl Into<Bake>, value: impl Into<Bake>) -> Self {
        self.custom_data_attrs_mut()
            .map
            .insert(attr.into(), value.into());
        self
    }
}

/// Inline JavaScript event handlers (`on*` attributes).
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
///
/// ```askama
/// {%- for (event, handler) in map.iter() %} on{{ event }}="{{ handler }}"{% endfor -%}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct EventHandlers {
    map: IndexMap<Bake, Bake>,
}

impl HasEventHandlers for &mut EventHandlers {
    fn event_handlers_mut(&mut self) -> &mut EventHandlers {
        self
    }
}

pub trait HasEventHandlers: Sized {
    fn event_handlers_mut(&mut self) -> &mut EventHandlers;

    fn on(mut self, event: impl Into<Bake>, handler: impl Into<Bake>) -> Self {
        self.event_handlers_mut()
            .map
            .insert(event.into(), handler.into());
        self
    }
}
