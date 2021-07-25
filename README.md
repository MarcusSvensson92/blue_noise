# Blue Noise

A blue noise generator using the void-and-cluster method. Made for learning. Written in Rust.

## Sources

* [Generating Blue Noise Textures With Void And Cluster](https://blog.demofox.org/2019/06/25/generating-blue-noise-textures-with-void-and-cluster/)
* [The void-and-cluster method for dither array generation](http://cv.ulichney.com/papers/1993-void-cluster.pdf)

## Example Command Line

```
cargo run --release noise.png 128 1.5
```