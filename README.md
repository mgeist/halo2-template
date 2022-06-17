# halo2 cargo template

A quick template using [cargo-generate](https://github.com/cargo-generate/cargo-generate) to get up and running with halo2 as quick as possible. Also includes a small util for generating circuit layout images.

Credit to https://github.com/icemelon/halo2-examples for the original `example.rs` script

### How to use:

- `cargo install cargo-generate` if you have not used `cargo generate` before
- `cargo generate --git https://github.com/mgeist/halo2-template`
- You'll be prompted to input a project name, you can name it whatever you like.
- You'll see something like `Done! New project created ...", which is where the project now lives

That's it. Now you're ready to go:

- `cargo run --bin example`

After running the example, you should see an `example.png` in your project root.

You can find the example circuit in `./src/bin/`

To use the helper util for drawing circuit layout images:

``` rust
use {{crate_name}}::util::DrawConfig;

let draw_config = DrawConfig::new("example.png").with_title("Example");

CircuitLayout::default()
    .render(k, &ciruit, &draw_config.build())
    .unwrap();
```