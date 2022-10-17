use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn lex_file(path: String) -> Result<Vec<String>, String> {
    let contents = std::fs::read_to_string(path);
    if let Err(_) = contents {
        return Err("Could not open file".to_string());
    }

    let input = contents.unwrap().replace("\n", " ");
    let parts: Vec<&str> = input.split(' ').collect();

    let mut numbers: Vec<String> = vec![];
    for part in parts {
        if part.len() > 0 && part.chars().all(char::is_alphanumeric) {
            numbers.push(part.to_string())
        }
    }

    Ok(numbers)
}

/// Taking an hexagonal color code: `3d45aa` to decimal RGB code
pub fn pixel_to_color(pixel: String) -> (u8, u8, u8, u8) {
    use std::i64;
    let red = i64::from_str_radix(pixel[..2].to_string().as_str(), 16).unwrap() as u8;
    let green = i64::from_str_radix(pixel[2..4].to_string().as_str(), 16).unwrap() as u8;
    let blue = i64::from_str_radix(pixel[4..6].to_string().as_str(), 16).unwrap() as u8;

    let alpha: u8 = if pixel.len() > 6 {
        i64::from_str_radix(pixel[6..8].to_string().as_str(), 16).unwrap() as u8
    } else {
        255
    };

    (red, green, blue, alpha)
}

pub fn render(path: String) -> Result<(), String> {
    // Lexing and parsing of file
    println!("Info: loading file: {}", path);
    let res = lex_file(path.clone());
    if let Err(str) = res {
        return Err(str);
    }
    let pixels = res.unwrap();
    let width = pixels[0].parse::<u32>().unwrap();
    let height = pixels[1].parse::<u32>().unwrap();

    if width * height > (pixels.len() - 2) as u32  {
        println!("Warning: too many pixels for the frame size.\n\tOnly using the ones within the frame pixel count");
    }

    // Initializing SDL and the canvas
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
    .window(path.as_str(), width, height)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    canvas.clear();

    // Filling up the canvas
    println!("Info: Rendering has start");
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    for pixel in pixels.iter().skip(2) {
        let colors = pixel_to_color(pixel.to_string());
        canvas.set_draw_color(Color::RGBA(colors.0, colors.1, colors.2, colors.3));
        if let Err(str) = canvas.draw_point(Point::new(x as i32, y as i32)) {
            return Err(str);
        }
        x += 1;
        if x == width {
            x = 0;
            y += 1;
        }
    }

    println!("Info: Rendering has finished");
    canvas.present();

    // Event and window event Loop
    println!("Info: Close windows or press ESC to stop program");
    let mut events = sdl_context.event_pump()?;
    'mainloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }
    println!("Exiting Program...");
    Ok(())
}

fn main() {
    // Prints each argument on a separate line
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Please enter a file location")
    } else if args.len() > 2 {
        println!("Only one argument allowed. Type: python3 wifee.py --help, for help")
    } else if args[1] == "--help".to_string() {
        println!("WIF Help:\n\tUsage: python3 wif.py <file>")
    } else if args[1][args[1].len()-4..] != ".wif".to_string() {
        println!("Expected a *.wif file")
    }

    if let Err(str) = render(args[1].to_string()) {
        println!("Error during rendering: {}", str);
    }
}
