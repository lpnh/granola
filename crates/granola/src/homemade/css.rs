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
/// .tooltip {
///   display: inline flex;
///   anchor-scope: all;
///   anchor-name: --tip;
/// }
/// .tip {
///   display: inline flex;
///   align-items: center;
///   justify-content: center;
///   width: 1.15rem;
///   height: 1.15rem;
///   padding: 0;
///   border: 1px solid;
///   border-color: var(--color-border);
///   border-radius: 50%;
///   background-color: var(--color-surface);
///   color: color-mix(in oklab, var(--color-text) 60%, #0000);
///   font-size: 0.7rem;
///   font-style: italic;
///   font-weight: 700;
///   line-height: 1;
///   cursor: help;
/// }
/// .tip:hover, .tip:focus-visible {
///   color: var(--color-text);
///   border-color: var(--color-primary);
/// }
/// .tip-bubble {
///   display: none;
///   opacity: 0;
///   transition: opacity 160ms ease, display 160ms allow-discrete;
///   position: fixed;
///   position-anchor: --tip;
///   anchor-name: --tip-bubble;
///   position-area: top;
///   margin-bottom: 0.5rem;
///   position-try-fallbacks: flip-block;
///   width: max-content;
///   max-width: 20rem;
///   padding: 0.25rem 0.5rem;
///   border-radius: 0.5rem;
///   background-color: var(--color-text);
///   color: var(--color-background);
///   font-size: 0.8rem;
///   font-style: normal;
///   font-weight: 400;
///   line-height: 1.25;
///   text-align: center;
///   white-space: normal;
///   pointer-events: none;
///   z-index: 2;
/// }
/// .tip-bubble::before {
///   content: "";
///   z-index: -1;
///   position: fixed;
///   position-anchor: --tip;
///   position-area: top;
///   width: 0.6rem;
///   height: 0.6rem;
///   background-color: var(--color-text);
///   transform: rotate(45deg);
///   margin-bottom: 0.35rem;
///   margin-top: calc(anchor-size(--tip-bubble block) - 0.3rem);
///   position-try-fallbacks: flip-block;
/// }
/// .tooltip:hover .tip-bubble, .tooltip:has(:focus-visible) .tip-bubble {
///   display: block;
///   opacity: 1;
/// }
/// @starting-style {
///   .tooltip:hover .tip-bubble, .tooltip:has(:focus-visible) .tip-bubble {
///     opacity: 0;
///   }
/// }
/// .tooltip-top .tip-bubble {
///   position-area: top;
///   margin-bottom: 0.5rem;
/// }
/// .tooltip-bottom .tip-bubble {
///   position-area: bottom;
///   margin-bottom: 0;
///   margin-top: 0.5rem;
/// }
/// .tooltip-left .tip-bubble {
///   position-area: left;
///   margin-bottom: 0;
///   margin-right: 0.5rem;
///   position-try-fallbacks: flip-inline;
/// }
/// .tooltip-right .tip-bubble {
///   position-area: right;
///   margin-bottom: 0;
///   margin-left: 0.5rem;
///   position-try-fallbacks: flip-inline;
/// }
/// .tooltip-top .tip-bubble::before {
///   position-area: top;
///   margin-bottom: 0.35rem;
///   margin-top: calc(anchor-size(--tip-bubble block) - 0.3rem);
/// }
/// .tooltip-bottom .tip-bubble::before {
///   position-area: bottom;
///   margin-top: 0.35rem;
///   margin-bottom: calc(anchor-size(--tip-bubble block) - 0.3rem);
/// }
/// .tooltip-left .tip-bubble::before {
///   position-area: left;
///   margin-top: 0;
///   margin-bottom: 0;
///   margin-right: 0.35rem;
///   margin-left: calc(anchor-size(--tip-bubble inline) - 0.3rem);
///   position-try-fallbacks: flip-inline;
/// }
/// .tooltip-right .tip-bubble::before {
///   position-area: right;
///   margin-top: 0;
///   margin-bottom: 0;
///   margin-left: 0.35rem;
///   margin-right: calc(anchor-size(--tip-bubble inline) - 0.3rem);
///   position-try-fallbacks: flip-inline;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Garnish;

impl StylesheetRecipe for Garnish {
    fn statements_recipe() -> Bake {
        let mut statements = bake![CssRule::from(Colors)];
        statements.fold_in_ws(CssRule::from(AnchorDefaults));
        statements.fold_in(Btn::statements_recipe());
        statements.fold_in(Tooltip::statements_recipe());
        statements
    }
}
