#![allow(dead_code)]

fn main()
{
  //if_statemets();
  //while_and_loop();
  //for_loop();
  match_statement();
}

fn if_statemets()
{
  let temp = 35;

  if temp > 30
  {
    println!("It's really hot outside.");
  }
  else if temp < 10
  {
    println!("Pretty cool outside.");
  }
  else
  {
    println!("Temp is OK.");
  }

  // If to init variables.
  let day = if temp > 20 {"sunny"} else {"cloudy"};
  println!("Today is {}.", day);

  // Inlined in functions or macros.
  println!("It is {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

  println!("It is 2 {}",
    if temp > 20 {
      if temp > 30 {"Very hot"} else {"Hot"}
    } else if temp < 10 {"cold"} else {"OK"});

}

fn while_and_loop()
{
  let mut x = 1;
  while x < 1000
  {
    x *= 2;

    if x == 64
    {
      continue;
    }
    println!("X is equal to {}", x);
  }

  let mut y = 1;
  loop
  {
    y *= 2;
    println!("Y is equal {}", y);

    if y == 1<<10
    {
      break;
    }
  }
}

fn for_loop()
{
  for x in 1..11
  {
    if x == 3 { continue; }

    if x == 8 { break; }
    println!("x = {}", x);
  }

  for (pos,y) in (30..41).enumerate()
  {
    println!("Value {} is in position {}", y, pos);
  }
}

fn match_statement()
{
  let country_code = 999;  // 1 to 999

  let country = match country_code
  {
    44 => "UK",
    46 => "Sweden",
    7 => "Russia",
    1...999 => "Unknown", // 1..99 == exclusive.  1...99 == inclusive.
    _ => "Invalid"
  };

  println!("The country with code {} is {}", country_code, country);
}
