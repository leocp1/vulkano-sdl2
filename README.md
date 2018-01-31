# vulkano-SDL2

A safe link between [`vulkano`](https://github.com/vulkano-rs/vulkano/tree/master/vulkano-win)
and the [`sdl2`](https://github.com/Rust-SDL2/rust-sdl2) library which can create a window to render to.

Attempts to mostly match [`vulkano-win`](https://github.com/vulkano-rs/vulkano/tree/master/vulkano-win),
but the surface is returned directly instead of as a field in a struct,
since `SDL2_vulkan` requires a window to return required instance extensions.
Please see `tests/create_surface.rs` as an example.

The `sdl2-vulkan-sys` directory binds the `SDL2_vulkan.h` functions.
