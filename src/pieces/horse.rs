use std::{collections::HashMap, rc::Rc};

use graphics::Transformed;
use opengl_graphics::{GlGraphics, Texture};
use piston::RenderArgs;

use crate::{Piece, TableVec};
#[derive(Clone)]
pub struct Horse {
  _selected: bool,
  texture: Rc<Texture>,
}

impl Horse {
  pub fn new(textures: &HashMap<&str, Rc<Texture>>) -> Box<Self> {
    let texture = textures.get("horse_green").unwrap().clone();
    Box::new(Self {
      _selected: false,
      texture,
    })
  }
}
impl Piece for Horse {
  fn draw(
    &self,
    args: &RenderArgs,
    gl: &mut GlGraphics,
    (x, y): (usize, usize),
  ) {
    gl.draw(args.viewport(), |c, gl| {
      let transformers_o_segundo_filme =
        c.transform.trans((y * 50) as f64, (x * 50) as f64);
      graphics::image(&*self.texture, transformers_o_segundo_filme, gl)
    });
  }
  fn is_position_valid(
    &self,
    current_pos: (usize, usize),
    desired_pos: (usize, usize),
    table: &TableVec<dyn Piece>,
  ) -> bool {
    let mut allowed_positions: Vec<(usize, usize)> = Vec::new();

    let (cx, cy) = current_pos;

    let possupreme = (cx + 1, cy + 2);
    let possupreme2 = (cx + 2, cy + 1);
    allowed_positions.push(possupreme);
    allowed_positions.push(possupreme2);

    if table[desired_pos.0][desired_pos.1].is_some() {
      return false;
    }

    if cy >= 2 {
      let pos1 = (cx + 1, cy - 2);
      allowed_positions.push(pos1);

      if cx >= 1 {
        let pos4 = (cx - 1, cy - 2);
        allowed_positions.push(pos4);
      }
    }
    if cx >= 1 {
      let pos1 = (cx - 1, cy + 2);
      allowed_positions.push(pos1);
    }
    if cy >= 1 {
      let pos2 = (cx + 2, cy - 1);
      allowed_positions.push(pos2);
    }
    if cx >= 2 {
      let pos1 = (cx - 2, cy + 1);
      allowed_positions.push(pos1);
      if cy >= 1 {
        let pos2 = (cx - 2, cy - 1);
        allowed_positions.push(pos2);
      }
    }

    if allowed_positions.contains(&desired_pos) {
      true
    } else {
      false
    }
  }
  fn clone_piece(&self) -> Box<dyn Piece> {
    Box::new(self.clone())
  }
}
