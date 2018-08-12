#![allow(dead_code)]
use std::mem;

fn main()
{
  //structures();
  //enums();
  //option();
  array();
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
    Color::CmykColor{cyan:_, magenta:_, yellow:_, black:_} => println!("Cmyk"),
    // Color::CmykColor{cyan:_, magenta:_, yellow:_, black:_} => println!("Cmyk ({}, {}, {}, {})",
    //   Color::CmykColor.cyan,
    //   Color::CmykColor.magenta,
    //   Color::CmykColor.yellow,
    //   Color::CmykColor.black)
    //_ => println!("Some other colour")
  }
}

fn array()
{
  // :[i32;5] is redundant.  We can do without it.
  let mut a:[i32; 5] = [1, 2, 3, 4, 5];

  println!("a has {} elements, first is {}",
    a.len(), a[0]);

  a[0] = 321;

  println!("a at [0] is {}", a[0]);
  println!("{:?}", a);

  if a != [1,2,3,4,5]
  {
    println!("Does not match.");
  }

  if a == [321,2,3,4,5]
  {
    println!("Match.");
  }

  let b = [1u8; 10];

  for i in 0..b.len()
  {
    println!("b array at index {} == {}", i, b[i]);

  }

  println!("b took up {} bytes", mem::size_of_val(&b));

  let mtx:[[f32;3]; 2] =
  [
    [1.0, 0.0, 0.0],
    [0.0, 2.0, 0.0]
  ];

  println!("Mtx {:?}", mtx);

  for i in 0..mtx.len()
  {
    for j in 0..mtx[i].len()
    {
      if i == j
      {
        println!("mtx[{}][{}] == {}", i, j, mtx[i][j]);
      }
    }
  }
}

fn option()
{
  // Option<T>

  let x = 3.0;
  let y = 2.0;

  // Some (z) None ()
  let result:Option<f64> =
    if y != 0.0 { Some(x / y) } else { None };

  println!("Result = {:?}", result);

  match result
  {
    Some(z) => println!("{} / {} = {}", x, y, z),
    None => println!("Can't devide {} by {}", x, y)
  }

  // if let / while let
  if let Some(z) = result { println!("z = {}", z); }
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
