# Linux

### Linux dependencies:
*libxtst-dev*

### Linux caveats

Currently the linux implementation will sleep for 100ms upon first invocation of the library.
Otherwise some initial key strokes are missed.

##### cross development linux -> windows
cross rulez.

to cross compile windows on linux:
```
argo install cross
cross check --target x86_64-pc-windows-gnu
```
