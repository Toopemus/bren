# Bren

A simple 3D software-rendering library for graphics in the terminal created with no knowledge on actual graphics programming.

check `examples/` for how to use the library.

```bash
# Run programs from the examples folder:
cargo run --example cube

# Generate and open documentation
cargo doc --open
```

## Todos

- [ ] Refactoring, less confusing division of responsibilities, **especially** when applying perspective transformations
- [ ] Z-buffer
- [ ] Shaders for objects? E.g. user passing a closure/function to the renderer that runs on each vertex
- [ ] Procedural generation of more geometry-primitives. Cube, sphere, etc.
- [ ] Support for parsing more 3D model formats

