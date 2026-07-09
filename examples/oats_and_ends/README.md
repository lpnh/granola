# oats and ends example

Serves the "Oats & Ends" café landing page, built from the shared
`oats-and-ends` crate that the snapshot tests also consume. One page
definition, two consumers: this example renders it, `tests/pretty` pins it.

Uses [axum](https://docs.rs/axum/latest/axum/) and
[tokio](https://docs.rs/tokio/latest/tokio/).

To run the example, use the command `cargo run -p oats_and_ends_example`. Once
running, open `http://127.0.0.1:8080/` in your browser.
