# Wouter's Image Format

Wouter's image format or WIF is a type of image format specifically made to be extremally easy to **create**, **implement** and **read**. It is mainly meant for simple visualizations like heatmaps, but can basicly do everything as a format like [JPEG](https://en.wikipedia.org/wiki/JPEG). The files be quite large due to the simple nature of this image format, maybe 3 to 4 times larger than I.E. JPEG.

**WARNING**: *This is still in development, and thus may contain some bugs, or other inconveniences*

The image specification can be found [below](#specification).

The goal is to have multiple generators that can visualize this image format, currently there are two: [Plain Javascript](/engine-web) and a [Python3 script](/engine-python). The next step is to have a native compiled visualizer, probably written in Rust or C++.

There is also a website where you can generate and look the final result, it makes use of the JavaScript engine from this repository. You can find it at: [wouterpennings.com/wif](https://wouterpennings.com/wif)

Currently, there are a couple of examples, and they are in the [examples](/examples/) directory. (These are not very impressive though :-)) 

## Specification

The image specification is very simple, and consists of two parts:
 1. Defining the size of the image, sometimes refered to as the "frame". You define the width and the height, both in pixels, seperated by whitespace.
 2. The second part is a list of hexadecimal RGB color codes; one for each pixel, formatted like this: `ff5733`. These are also seperated by whitespace. The pixel color codes are sorted by going horizontal, left to right, from top to bottom. The first code is the top-left pixel of the image, and the last one is the bottom-right pixel of the image.

When refering to whitespace, there is no limit, as long as there is at least one. Of course, more whitespace does not add anything, it only makes the file larger.

The template for the *.wif files becomes: 
```
<width in pixels>  <height in pixels>  pixel1  pixel2  pixel3  pixel4 ......
```

### Example

Lets say we want an image of an 3x3 pixels, white and black pixels, like a chessboard. You want to get something like this (B = black, W = white):

```
B W B
W B W
B W B
```

Color code for black: `000000`, and white: `ffffff`. The file should look something like this:

```
3 3 000000 ffffff 000000 ffffff 000000 ffffff 000000 ffffff 000000
```