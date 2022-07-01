# tampon[^1]
Contains SAFE Rust functions, macros and trait to generate and manage buffers.

#### Functions
1. [`generate_buffer(...) -> Vec<u8>`](https://docs.rs/tampon/1.0.0/tampon/fn.generate_buffer.html) - Generate a random buffer with specific size and charset.
2. [`wipe_buffer(...)`](https://docs.rs/tampon/1.0.0/tampon/fn.generate_buffer.html) - Wipe the buffer, overwritting it with zeroes.
3. [`compare_buffer(...)`](https://docs.rs/tampon/1.0.0/tampon/fn.generate_buffer.html) - Compare 2 buffers and return the absolute difference between them.

#### Macros
1. [`to_buffer!(...)`](https://docs.rs/tampon/1.0.0/tampon/fn.generate_buffer.html) - Variadic macro that copy multiple scalars, vectors and implementator of trait Tampon into a buffer.
2. [`from_buffer!(...)`](https://docs.rs/tampon/1.0.0/tampon/fn.generate_buffer.html) - Variadic macro that copy buffer into multiple scalars, vectors and implementator of trait Tampon.
3. [`buffer!(...) -> Vec<u8>`](https://docs.rs/tampon/1.0.0/tampon/fn.generate_buffer.html) - Variadic macro that create a buffer from multiple scalars, vectors and implementator of trait Tampon.
4. [`bytes_size!(...) -> usize`](https://docs.rs/tampon/1.0.0/tampon/fn.generate_buffer.html) - Variadic macro that return the total size in bytes of multiple scalars, vectors and implementator of trait Tampon.

[^1]: [`Tampon`](https://www.google.com/search?q=memory+buffer+in+french) means `buffer` in french.