use bit_vec::BitVec;

#[cfg_attr(rustfmt, rustfmt_skip)]
const NEIGHBORS: &[(isize, isize)] = &[
  (-1, -1), (0, -1), (1, -1),
  (-1,  0),          (1,  0),
  (-1,  1), (0,  1), (1,  1),
];

#[derive(Debug)]
pub struct World {
  pub width: usize,
  pub height: usize,
  data: BitVec,
}

impl World {
  pub fn new(width: usize, height: usize) -> Self {
    World {
      width,
      height,
      data: BitVec::from_elem(width * height, false),
    }
  }

  pub fn get(&self, x: usize, y: usize) -> bool {
    self.assert_in_bounds(x, y);
    self.data.get(y * self.width + x).unwrap()
  }

  pub fn set(&mut self, x: usize, y: usize, cell: bool) {
    self.assert_in_bounds(x, y);
    self.data.set(y * self.width + x, cell);
  }

  fn assert_in_bounds(&self, x: usize, y: usize) {
    debug_assert!(
      x < self.width,
      "x out of bounds: the width is {} but the x is {}",
      self.width,
      x
    );

    debug_assert!(
      y < self.height,
      "y out of bounds: the height is {} but the y is {}",
      self.height,
      y
    );
  }

  pub fn next_generation(&self) -> Self {
    let mut next_world = World::new(self.width, self.height);

    for y in 0..self.height {
      for x in 0..self.width {
        let cell = self.get(x, y);

        let n = self.count_neighbors(x, y);
        let next_cell = if cell { n >= 2 && n <= 3 } else { n == 3 };

        next_world.set(x, y, next_cell);
      }
    }

    next_world
  }

  fn count_neighbors(&self, x: usize, y: usize) -> u8 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn checked_add(n: usize, dir: isize, max: usize) -> Option<usize> {
           if dir == 0               { Some(n)     }
      else if dir < 0 && n > 0       { Some(n - 1) }
      else if dir > 0 && n < max - 1 { Some(n + 1) }
      else                           { None        }
    }

    let mut result = 0;
    for (dx, dy) in NEIGHBORS {
      let dx = checked_add(x, *dx, self.width);
      let dy = checked_add(y, *dy, self.height);
      match (dx, dy) {
        (Some(dx), Some(dy)) if self.get(dx, dy) => result += 1,
        _ => {}
      }
    }
    result
  }

  pub fn render(&self) -> String {
    let mut result = String::with_capacity((self.width + 1) * self.height);
    for row in (0..self.height).map(|y| self.render_row(y)) {
      result.push_str(&row);
      result.push('\n');
    }
    result
  }

  pub fn render_row(&self, y: usize) -> String {
    (0..self.width)
      .map(|x| if self.get(x, y) { '#' } else { '~' })
      .collect()
  }
}
