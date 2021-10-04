//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
  assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input() -> Universe {
  let mut universe = Universe::new();
  universe.set_width(6);
  universe.set_height(6);
  universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
  universe
}

#[cfg(test)]
pub fn expected() -> Universe {
  let mut universe = Universe::new();
  universe.set_width(6);
  universe.set_height(6);
  universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
  universe
}

#[wasm_bindgen_test]
pub fn tick() {
  let mut input = input();
  let expected = expected();
  input.tick();
  assert_eq!(&input.get_cells(), &expected.get_cells());
}
