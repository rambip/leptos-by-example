# Adding an example
Imagine you have an example named `foo`

1) go inside `examples`.

## create `foo.rs`
This rust file has to define a `showcase() -> impl IntoView` function

## create `foo.toml`
Inside this file, you define:
- a `description: String` field.
This is one sentence describing the example, a bit longer than a commit message
You cannot use markdown in this field

- a `motivation: String` field
This is one paragraph that explain to the reader why you wrote this example.
Is is to illustrate some functionnality of leptos ?
to show some usefull pattern ?
to provide a specific component ?

You can use markdown, but without headers.

- a `features: Vec<String>` field: the leptos features that must be declared in cargo for the project to compile.
You can ommit the `csr` feature since it is enabled inside each example

optionnaly, a `related` field to provide links to documentation.
This can be internal, like `[counter](./counter)`
Or external, like `<https://https://leptos-rs.github.io/leptos/>`

optionnaly a `dependencies` field for other dependencies


## add dependencies
If you need specific dependencies, add them to `Cargo.toml`

## Add `foo.css`
If your example needs a specific stylesheet, add a `foo.css` file with the content you want.
It is not supported right now.


# How it works
Look at `build.rs` and the generated `src/examples.rs`, it should make sense
