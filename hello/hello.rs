use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
  // This trait requires `fmt` with this exact signature.Structure
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write strictly the first element into the supplied output
    // stream: `f`. Returns `fmt::Result` which indicates whether the
    // operation succeeded or failed. Note that `write!` uses syntax which
    // is very similar to `println!`.
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`
impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Use `self.number` to refer to each position data point.
    write!(f, "({}, {})", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64,
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} + {}i", self.real, self.imag)
  }
}

fn main() {
  let minmax = MinMax(0, 14);

  println!("Compare structures:");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);

  println!("The big range is {big} and the small is {small}",
            small=small_range,
            big=big_range);

  let point = Point2D{x:3.3, y: 7.2};

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  let complex = Complex{real: 3.3, imag: 7.2};

  println!("Display: {}", complex);
  println!("Display: {:?}", complex);
}