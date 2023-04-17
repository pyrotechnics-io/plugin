
# Rust Plugins

This code demonstrates how to build and load plugins in Rust

## Building

You can build the library like so:
```bash
cargo build
```

To build the sample code:

```bash
cargo build --examples
```

## Running

Copy the built library to /tmp
```bash
cp target/debug/libplugin.so /tmp/
```
 > This is done so that our binary will find and load the library from a well known and unambiguous path

Run the example like so:

```bash
./target/debug/examples/tester
```
