# Wouter's Image Format

Wouter's image format or WIF is a type of image format specifically made to be extremally easy to **create**, **implement** and **read**. It is mainly meant for simple visualizations like heatmaps, but can basicly do everything as a format like [JPEG](https://en.wikipedia.org/wiki/JPEG). The files be quite large due to the simple nature of this image format, maybe 30 to 40 times larger than I.E. JPEG.

**WARNING**: *This is still in development, and thus may contain some bugs, or other inconveniences*

The goal is to have multiple generators that can visualize this image format, currently there are three:
- [Plain Javascript](/engine-web), using HTML canvas
- [Python3 Script](/engine-python), using OpenCV Python Library
- [Rust CLI](/engine-rust), using SDL2 with OpenGL under the hood

There is also a website where you can generate and look the final result, it makes use of the JavaScript engine from this repository. You can find it at: [wouterpennings.com/wif](https://wouterpennings.com/wif)

Currently, there are a couple of examples, and they are in the [examples](/examples) directory. The scripts with which the examples are generate are also in the same folder.

## Specification

You can find the specification of the image format at: [./specification.md](specification.md),
