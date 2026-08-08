# cuisine example

A simple example about layout/styling, using
[axum](https://docs.rs/axum/latest/axum/),
[tokio](https://docs.rs/tokio/latest/tokio/), and
[tower](https://docs.rs/tower/latest/tower/).  

The palette page generates a [daisyUI](https://daisyui.com/) base color scale.
Picking a color derives `base-100`, `base-200`, `base-300`, and `base-content`,
which are then applied over the active theme.

To run the example, use the command `cargo run -p cuisine`. Once running,
open `http://127.0.0.1:8080/` in your browser.
