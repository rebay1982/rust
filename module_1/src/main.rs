#![allow(dead_code)]

use std::mem;
mod sh;

const MEANING_OF_LIFE:u8 = 42;  // No fixed address, think "define" in C
static mut Z:i32 = 123;

fn main()
{
  sh::stack_and_heap();
  //operators();
  //scope_and_shadowing();
  //fundamental_data_types();
  //consts();
}

fn operators()
{
  // Arithmetic
  let mut a = 2+3*4;
  println!("{}", a);

  // -- and ++ don't exist.
  a -= 2;
  a += 1;

  println!("remainder of {} / {} = {}", a, 3, a%3);

  // a ^ 3;
  let a_cubed = i32::pow(a, 3);
  println!("a cubed = {}", a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("b = {}, b cubed = {}, b to pi = {}", b, b_cubed, b_to_pi);

  // bitwise operators for integers only
  let c = 1 | 2; // | OR & AND ^ XOR ! NOR
  println!("c = {}", c);

  // Shift operators on bits
  let two_to_10 = 1 << 10;
  println!("2 ^ 10 = {}", two_to_10);

  // Logical operators
  let pi_less_4 = std::f64::consts::PI < 4.0; //true;
  let x = 5;
  let x_is_5 = x == 5;

  println!("pi less than 4 = {}", pi_less_4);
  println!("x is 5 = {}", x_is_5);

}

fn scope_and_shadowing()
{
  let a = 123;

  {
    let b = 456;
    let a = 789;
    println!("inside b = {}", b);
    println!("inside a = {}", a);
  }

  println!("outside a = {}", a);
}

fn fundamental_data_types()
{
  let a:u8 = 123; // 8 bits, u for unsigned? yep.
  println!("a = {}", a);

  let mut b:i8 = 0;
  println!("b = {}", b);

  b = 1;
  println!("b = {}", b);

  let c = 123456789;
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

  let z:isize = 123;
  let size_of_z = mem::size_of_val(&z);

  println!("z = {}, takes up {} bytes. {}-bit os", z, size_of_z, size_of_z * 8);

  // :char is optional, type will be inferred.
  let d:char = 'x';
  println!("d = {}, takes up {} bytes", d,  mem::size_of_val(&d));


  let e:f32 = 2.5; // double-precision, 8bytes or 64bits, f64.
  println!("e = {}, takes up {} bytes", e,  mem::size_of_val(&e));

  let f = true;
  println!("f = {}, takes up {} bytes", f,  mem::size_of_val(&f));

  let g:bool = 4 > 0;
  println!("g = {}, takes up {} bytes", g,  mem::size_of_val(&g));
}

fn consts()
{
  unsafe
  {
    println!("{}", MEANING_OF_LIFE);
    println!("{}", Z);
  }

}
