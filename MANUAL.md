# Manual
This is the official guidebook on how to add your custom file 
generation template into the executable. It is recommended to use the "Build from the source method" to perform these modifications. 

## Important Notes
User's extensions take precedence over built-in language modules. So, if an extension has `--cpp` as a key, it would take the priority instead of
built-in `--cpp` argument. However, the main 4 utils command have the highest priority, meaning no extensions can "override" it or take
precedence.

## Step 1
Step 1, you may navigate to the [custom folder](./src/languages/custom) inside src/languages, and you can find mod.rs for the custom registry.
You can also find `example.rs` as an existing template with the documentation on how you can insert or add your own template. 

Inside of `mod.rs`:
```rust
/* You can add or append pub(super) mod your_boilerplate_name; so the register and the progam
   recognize it (NECESSARY) */
pub(super) mod example;

use std::collections::HashMap;

type Generator = fn() -> &'static str;

pub fn registry() -> HashMap<&'static str, Generator> {
    HashMap::from([
        /* You can add or append your boilerplate to the registry with the following format
           "--example" for the option or argument to invoke it
           "example::generate" for the invocation of the boilerplate template you've provided, 
           you may change the "exanple" to match your file name that defines the boilerplate 
           (NECESSARY) */
        ("--example", example::generate as Generator), 
    ])
}
```

Inside of `example.rs`:
```rust
// This is an example, you can just copy this file and modify to your liking
// You can also remove these comments
pub(super) fn generate() -> &'static str {
// You can add yoour custom boilerplate in between the r#" "#
// Example:
// r#"Top
// Middle
// Bottom"#
r#"

"#
}
```

## Step 2
After you found two of the files and verify the content, you can now copy the `example.rs` and rename it into `your_file.rs`
with `snake_case` as the naming convention. In this case, I'd name it as `my_rust.rs` 

## Step 3
After copying, you can look at inside the file (in this case, it's `my_rust.rs`) and edit the paragraph between `r#"` and `"#` to add your custom
boilerplate. You can also remove the comments if you think it's a clutter (the ones started with `/`). For example:
```rust
pub(super) fn generate() -> &'static str {
r#"fn main() {
    println!("Hello world!");
}"#
}
```

It doesn't have to be a boilerplate, so feel free to use it to generate your own template or artifact as long as it's valid
for the interfaces!

## Step 4
You can load your own template to the dedicated registry built for user extension, it's located on [src/languages/custom/mod.rs](./src/languages/custom/mod.rs).
You can do it by adding `pub(super) mod your_filename_without_file_extension;` on top of the `mod.rs` like this:
```rust
pub(super) mod example;
pub(super) mod my_rust;

use std::collections::HashMap;

type Generator = fn() -> &'static str;

pub fn registry() -> HashMap<&'static str, Generator> {
    HashMap::from([
        ("--example", example::generate as Generator), 
    ])
}
```

## Step 5
Now we get into the most crucial part, after loading your own template to the registry, you have to register your template into the registry by appending an entry
to the pre-existing HashMap, for example:
```rust
pub(super) mod example;
pub(super) mod my_rust;

use std::collections::HashMap;

type Generator = fn() -> &'static str;

pub fn registry() -> HashMap<&'static str, Generator> {
    HashMap::from([
        ("--example", example::generate as Generator), 
        ("--my_rust", my_rust::generate),
    ])
}
```
Here I don't need to add "as Generator" since the first entry already has it, but if you are going to remove `example`, you may need to add `as Generator` to your top-most entry. 
You also need to add a comma after every entry even for the last one. If we look at the code further, `("--my_rust", my_rust::generate)`, each part has its own functionality.
An entry is defined or started with open parenthesis and ended with close parenthesis with comma as a separator between entries. `"--my_rust"` here defines the "key" or the "command" or specific argument so you can run it with `elixare --my_rust file.rs`, 
while `my_rust::generate` is defined as `mod_name::function` which basically call a function from a specific module (in this case, generate from my_rust.rs) to run.

## Step 6
The last step, you can run:
```shell
cargo install --path
```
To build and install it.
