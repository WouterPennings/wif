# Specification

The image specification is very simple, and consists of two parts:

1. Defining the size of the image, sometimes refered to as the "frame". You define the width and the height, both in pixels, seperated by whitespace.
2. The second part is a list of hexadecimal [color codes](#color-codes); one for each pixel. These color codes are also
seperated by whitespace. The pixel color codes are sorted by going horizontal, left to right, from top to bottom.
The first code is the top-left pixel of the image, and the last one is the bottom-right pixel of the image.

When refering to whitespace, there is no limit, as long as there is at least one. Of course, more whitespace does not add anything, it only makes the file larger.

The template for the *.wif files becomes:
```
<width in pixels>  <height in pixels>  pixel1  pixel2  pixel3  pixel4 ......
```

If there are more pixels defined than the frame contains, it should not crash and just use up pixel color codes at the beginning of the list until the frame is filled. When there are too little pixel color codes, an image cannot be created.

## Color Codes

The color codes are either of type RGB (Red Green Blue) or RGBA (Red Green Blue Alpha). Each color seperate color is a hexadecimal 8 bit value; `00` to `ff`.
`0` is the lowest (color is off), `255` is the highet (color is fully on). When alpha is `0`, the pixel is fully transparant, at `255`, it is fully colored.
All the RGBA values are stuck together into one string of 6 or 8 characters, depending if a `alpha` value is stated. If `alpha` is **not** stated, it will default to `255` (or `ff` in hexadecimal).
Not every pixel needs to have an alpha value, it is optional per pixel basis.

Examples of color codes:
 - RGBA: `ff0000ff`
 - RBGA: `ff00aaff`
 - RGBA:`00ff80aa`
 - RGB: `ffffff`

## Example

Lets say we want an image of an 3x3 pixels, white and black pixels, like a chessboard. You want to get something like this (B = black, W = white):

```
B W B
W B W
B W B
```

Color code for black: `000000`, and white: `ffffff`. The file should look something like this, if you ignore the alpha values:

```
3 3 000000 ffffff 000000 ffffff 000000 ffffff 000000 ffffff 000000
```

or something like this when you explicitly put describe the alpha values:

```
3 3 000000ff ffffffff 000000ff ffffffff 000000ff ffffffff 000000ff ffffffff 000000ff
```