use crate::common::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
  width:  u32,
  height: u32,
  cells:  Vec<Cell>,
}

impl Display for Universe {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n");
    }
    Ok(())
  }
}

#[wasm_bindgen]
impl Universe {
  pub fn new() -> Self {
    let width = 64;
    let height = 64;

    let cells = (0..width * height)
      .map(|i| {
        if i % 2 == 0 || i % 7 == 0 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    Universe {
      width,
      height,
      cells,
    }
  }

  pub fn render(&self) -> String {
    self.to_string()
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.index(row, col);
        let cell = self.cells[idx];
        let count = self.count(row, col);

        let next_cell = match (cell, count) {
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          (Cell::Dead, 3) => Cell::Alive,
          (other, _) => other,
        };

        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  fn index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;

    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }
        count += self.cells[self.index(
          (row + delta_row) % self.height,
          (column + delta_col) % self.width,
        )] as u8;
      }
    }
    count
  }
}
