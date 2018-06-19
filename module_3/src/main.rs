#![allow(dead_code)]

fn main()
{
  structures();
  enums();
}

// Enumerations
enum Color
{
  Red,
  Green,
  Blue,
  RgbColor(u8, u8, u8),  // tuple
  CmykColor{ cyan: u8, magenta: u8, yellow: u8, black: u8} // struct
}

fn enums()
{
  let c:Color = Color::CmykColor { cyan:0, magenta: 128, yellow: 0, black: 0};

  match c
  {
    Color::Red => println!("r"),
    Color::Green => println!("g"),
    Color::Blue => println!("b"),
    Color::RgbColor(0, 0, 0) => println!("Black"),
    Color::RgbColor(r, g, b) => println!("RGB ({}. {}. {})", r, g, b),
    Color::CmykColor{cyan:_, magenta:_, yellow:_, black:_} => println!("Cmyk")
    // Color::CmykColor{cyan:_, magenta:_, yellow:_, black:_} => println!("Cmyk ({}, {}, {}, {})",
    //   Color::CmykColor.cyan,
    //   Color::CmykColor.magenta,
    //   Color::CmykColor.yellow,
    //   Color::CmykColor.black)
  }
}

// Structures
struct Point
{
  x: f64,
  y: f64
}

struct Line
{
  start: Point,
  end: Point
}

fn structures()
{
  let p = Point { x: 3.0, y: 4.0 };
  println!("point p is at ({}, {})", p.x, p.y);

  let p2 = Point { x: 5.0, y: 10.0 };

  let my_line = Line {start: p, end: p2};
  println!("line is from ({}, {}) to ({}, {})",
    my_line.start.x,
    my_line.start.y,
    my_line.end.x,
    my_line.end.y);
}
