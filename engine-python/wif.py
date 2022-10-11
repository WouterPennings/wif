import numpy as np
import cv2 as cv

def lex_input(input):
    parts = input.split(' ')
    return [part for part in parts if part.isalnum()]

def visualize(file):
    print(f"Info: loading {file}")
    input = open(file, 'r').read()
    numbers = lex_input(input)

    width = int(numbers[0])
    height = int(numbers[1])

    pixels = numbers[2:]
    if len(pixels) > width * height:
        print("Warning: too many pixels for the frame size.\n\tOnly using the ones within the frame pixel count")

    print("Info: drawing the image")
    img = np.zeros((width,height,3), np.uint8)
    x = 0
    y = 0
    for color in pixels:
        red   = int(color[:2], 16)
        green = int(color[2:4], 16)
        blue  = int(color[4:], 16)

        cv.rectangle(img,(x,y),(x+1, y+1),(red,green,blue),-1)

        x += 1
        if x == width:
            x = 0
            y += 1

    print("Info: Loading image windows")
    cv.imshow(file, img)

    print("Info: Close windows to stop program")
    wait_time = 1000
    while cv.getWindowProperty(file, cv.WND_PROP_VISIBLE) >= 1:
        keyCode = cv.waitKey(wait_time)
        if (keyCode & 0xFF) == ord("q"):
            cv.destroyAllWindows()
            exit()
    print("Exiting Program...")

if __name__ == "__main__":
    import sys
    args = sys.argv

    if len(args) == 1:
        print("Please enter a file location")
    elif len(args) > 2:
        print("Only one argument allowed. Type: python3 wifee.py --help, for help")
    elif args[1] == "--help":
        print("WIF Help:\n\tUsage: python3 wif.py <file>")
    elif not args[1][-4:] == ".wif":
        print("Expected a *.wif file")
    else:    
        visualize(args[1])
    