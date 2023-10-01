# Adding an example
Imagine you have an example named `foo`

1) go inside `examples`.

## create `foo.rs`
This rust file has to define a `showcase() -> impl IntoView` function

## create `foo.toml`
Inside this file, you define:
- a `description: String` field
- a `features: Vec<String>` field. You can ommit the `csr` feature since it is enabled inside each example

## add dependencies
If you need specific dependencies, add them to `Cargo.toml`

## Add `foo.css`
If your example needs a specific stylesheet, add a `foo.css` file with the content you want.
It is not supported right now.


# How it works
Look at `build.rs` and the generated `src/examples.rs`, it should make sense
