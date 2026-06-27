use crate::{homemade::*, prelude::*, recipes::*};

/// The homemade recipe for the [`CssStylesheet`].
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let stylesheet = CssStylesheet::from(Garnish);
///
/// assert_eq!(
///     stylesheet.bake_pretty(),
///     r#":root {
///   --color-background: initial;
///   --color-surface: initial;
///   --color-border: initial;
///   --color-text: initial;
///   --color-primary: initial;
///   --color-primary-text: initial;
///   --color-error: initial;
///   --color-success: initial;
/// }
/// a:not([class]) {
///   text-decoration-skip-ink: auto;
///   color: currentcolor;
/// }
/// .btn {
///   display: inline flex;
///   align-items: center;
///   justify-content: center;
///   padding: 0.6em 1.2em;
///   height: var(--btn-size);
///   font-size: 0.875rem;
///   font-weight: 500;
///   line-height: 1.25rem;
///   text-decoration: none;
///   white-space: nowrap;
///   border: 1px solid;
///   border-color: var(--btn-border);
///   border-radius: 0.5em;
///   background-color: var(--btn-bg);
///   color: var(--btn-fg);
///   cursor: pointer;
///   transition: background-color 150ms ease;
///   --btn-size: 2.5rem;
///   --btn-bg: var(--btn-color, var(--color-surface));
///   --btn-border: color-mix(in oklab, var(--btn-bg), #000 5%);
///   --btn-fg: var(--color-text);
/// }
/// .btn:hover {
///   --btn-bg: var(
///     --btn-hover-bg,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 7%)
///   );
/// }
/// .btn:active {
///   --btn-bg: var(
///     --btn-active-bg,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 5%)
///   );
///   --btn-border: var(
///     --btn-active-border,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 7%)
///   );
///   transform: scale(0.97);
/// }
/// .btn:focus-visible {
///   outline-width: 2px;
///   outline-style: solid;
///   outline-color: var(--btn-color, var(--color-text));
///   outline-offset: 2px;
/// }
/// .btn-primary {
///   --btn-color: var(--color-primary);
///   --btn-fg: var(--color-primary-text);
/// }
/// .btn-ghost {
///   color: var(--btn-color, var(--color-text));
///   --btn-bg: #0000;
///   --btn-border: #0000;
///   --btn-hover-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-text)) 10%,
///     #0000
///   );
///   --btn-active-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-text)) 20%,
///     #0000
///   );
///   --btn-active-border: #0000;
/// }
/// .btn-square {
///   padding: 0;
///   width: var(--btn-size);
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Garnish;

impl StylesheetRecipe for Garnish {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::from(Colors).into(),
            CssRule::from(AnchorDefaults).into(),
        ]);
        Btn::statements_recipe(statements);
    }
}
