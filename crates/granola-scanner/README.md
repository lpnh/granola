# granola-scanner

`granola-scanner` scans Rust source files for Granola daisyUI API usage and writes the corresponding CSS class names to a file for Tailwind CSS to consume.

## Motivation

Tailwind CSS discovers utility and component classes by scanning source files for exact string literals. In Granola, UI components and styles are often referenced through typed Rust paths, structs, and macros rather than raw class strings.

Without a dedicated scanner, there are two alternatives:

1. **Safelisting the full daisyUI catalog:** Including all daisyUI classes in a static safelist avoids missing styles, but it generates substantially larger stylesheets containing unused component rules. Furthermore, daisyUI modifier variants (such as `is-drawer-open:`) cannot be exhaustively safelisted because their combinations with utilities are unbounded.
2. **Manual safelisting:** Writing and maintaining an explicit list of required classes avoids stylesheet bloat, but requires continuous manual tracking whenever code changes.

`granola-scanner` automates class extraction by inspecting Rust syntax trees for Granola component and macro usage, keeping stylesheet output scoped to what the application actually references.

## Scanner Contract and Limitations

The scanner operates as a best-effort convenience tool rather than a full semantic analyzer:

- **False negatives:** The scanner parses syntax using `syn`. Classes referenced through dynamic string formatting, indirection, or third-party wrapper macros outside its resolution model may be missed. In these situations, missing classes should be listed manually in a Tailwind safelist.
- **False positives:** Unused or dead code containing Granola references will still emit class names if present in the scanned files.

## Built-in Component Safelist

`resources/safelist` contains the baseline set of concrete classes declared by Granola's built-in daisyUI components. `granola-scanner` updates this file during its build when the component list changes.
