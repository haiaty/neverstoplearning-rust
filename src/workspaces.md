To have a workspace the Cargo.toml should be:

```
[workspace]

members = [
    "main",
    "downloaders"
]

# here we can specify 
# what to run if nothing is specified.
# If you don't specify it the 'main'
# crate will be assumed

default-members = [
    "main"
]

```


If you want ro run only one crate member. run

```
cargo run -p <cratename>
```

To generate a new  new library crate:

```
cargo new <name> --lib
```