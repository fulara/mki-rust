# mki - mouse-keyboard-input 
Windows & Linux library for registring global input hooks and simulating keyboard and mouse events.

## Linux

### Linux dependencies:
*libxtst-dev*

### Linux caveats

Currently the linux implementation will sleep for 100ms upon first invocation of the library.
Otherwise some initial key strokes are missed.

##### cross development linux -> windows
cross rulez.

to cross compile windows on linux:
```rust
cargo install cross
cross check --target x86_64-pc-windows-gnu

```
