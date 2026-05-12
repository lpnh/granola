use askama::Template;
use std::borrow::Cow;

use crate::prelude::*;

/// A CSS property and a CSS value pair.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Introduction#css_declarations)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration = CssDeclaration::new("color", "rgb(102, 51, 153)");
///
/// assert_eq!(css_declaration.bake(),
/// "color: rgb(102, 51, 153);");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration: CssDeclaration = ("color", "rebeccapurple").into();
///
/// assert_eq!(css_declaration.bake(),
/// "color: rebeccapurple;");
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ property }}: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssDeclaration {
    pub property: Cow<'static, str>,
    pub value: Cow<'static, str>,
}

impl<P: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>> From<(P, V)> for CssDeclaration {
    fn from((property, value): (P, V)) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }
}

impl CssDeclaration {
    pub fn new(
        property: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }
}

/// A collection of [`CssDeclaration`].
///
/// The properties-list of [`CssRule`].
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration = CssDeclaration::new("color", "rgb(102, 51, 153)");
///
/// let css_properties_list = CssPropertiesList::new().push(css_declaration);
///
/// assert_eq!(css_properties_list.bake(),
/// "color: rgb(102, 51, 153);");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_declaration: CssDeclaration = ("color", "rebeccapurple").into();
///
/// let css_properties_list: CssPropertiesList = css_declaration.into();
///
/// assert_eq!(css_properties_list.bake(),
/// "color: rebeccapurple;");
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_properties_list: CssPropertiesList = [
///     ("color", "violet"),
///     ("font-weight", "lighter"),
/// ].into();
///
/// assert_eq!(css_properties_list.bake(),
/// "color: violet;
/// font-weight: lighter;");
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- for d in declarations -%}
/// {{- d -}}
/// {%- if !loop.last %}
/// {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPropertiesList {
    pub declarations: Vec<CssDeclaration>,
}

impl CssPropertiesList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, css_declaration: impl Into<CssDeclaration>) -> Self {
        self.declarations.push(css_declaration.into());
        self
    }

    pub fn new_declaration(
        mut self,
        property: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.declarations.push(CssDeclaration::new(property, value));
        self
    }
}

impl<D: Into<CssDeclaration>, const N: usize> From<[D; N]> for CssPropertiesList {
    fn from(items: [D; N]) -> Self {
        Self {
            declarations: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl<D: Into<CssDeclaration>> From<Vec<D>> for CssPropertiesList {
    fn from(items: Vec<D>) -> Self {
        Self {
            declarations: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<CssDeclaration> for CssPropertiesList {
    fn from(css_declaration: CssDeclaration) -> Self {
        Self {
            declarations: vec![css_declaration],
        }
    }
}

impl<P: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>> From<(P, V)> for CssPropertiesList {
    fn from(decl: (P, V)) -> Self {
        Self {
            declarations: vec![decl.into()],
        }
    }
}
