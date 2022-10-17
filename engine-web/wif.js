const isAlphaNumeric = str => /^[a-z0-9]+$/gi.test(str);

function lex_input(input) {
    let parts = input.split(' ');
    let parts2 = [];
    parts.forEach(element => {
        if(isAlphaNumeric(element)) {
            parts2.push(element)
        }
    });
    return parts2;
}

function render(input) {
    let numbers = lex_input(input);
    const width = parseInt(numbers.shift());
    const height = parseInt(numbers.shift());

    if(numbers.length > width * height) {
        console.log("Warning: too many pixels for the frame size.\n\tOnly using the ones within the frame pixel count");
    }

    let canvas = document.createElement('canvas');
    const ctx = canvas.getContext("2d");
    canvas.width = width*2;  
    canvas.height = height*2; 
    canvas.style.border = "black 1px solid";
    canvas.style.padding = "0px";
    canvas.id = "canvas";

    let index = 0;
    let size = 2;
    for(let i = 0; i < height; i++) {
        for(let j = 0; j < width; j++) {
            let red = numbers[index].substring(0, 2).replace(/^#/, '');
            let green = numbers[index].substring(2, 4).replace(/^#/, '');
            let blue = numbers[index].substring(4, 6).replace(/^#/, '');
            let alpha = 255
            if( len(numbers) === 8) {
                alpha = numbers[index].substring(6, 8).replace(/^#/, '');
            }
            
            ctx.fillStyle = "#"+red+green+blue+" ";
            ctx.fillRect(i*size, j*size, size, size)
            index++;
        }
    }

    return canvas;
}
