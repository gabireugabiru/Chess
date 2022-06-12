use std::{collections::HashMap, rc::Rc};

use graphics::Transformed;
use opengl_graphics::{GlGraphics, Texture};
use piston::RenderArgs;

use crate::{Piece, TableVec};
#[derive(Clone)]
pub struct Tower {
  texture: Rc<Texture>,
}

impl Tower {
  pub fn new(textures: &HashMap<&str, Rc<Texture>>) -> Box<Self> {
    let texture = textures.get("tower_green").unwrap().clone();
    Box::new(Self { texture })
  }
}
impl Piece for Tower {
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
    if current_pos == desired_pos {
      return false;
    }
    let (dx, dy) = desired_pos;
    let (cx, cy) = current_pos;
    let mut max_y = (7, 0);
    let mut max_x = (7, 0);

    //GET THE BOUNDS FOR X
    for i in 0..8 {
      if table[i][cy].is_some() {
        if cx == i {
          continue;
        } else if cx < i {
          max_x.0 = i - 1;
        } else {
          max_x.1 = i + 1;
          break;
        }
      }
    }
    //GET THE BOUNDS FOR Y
    for i in 0..8 {
      if table[cx][i].is_some() {
        if cy == i {
          continue;
        } else if cy < i {
          max_y.0 = i - 1;
        } else {
          max_y.1 = i + 1;
          break;
        }
      }
    }

    //CHECK IF IS THE SAME COLUMN OR SAME ROW
    if cx == dx || dy == cy {
      // CHECK IF IT IS IN THE BOUNDS
      if dx <= max_x.0 && dx >= max_x.1 {
        if dy <= max_y.0 && dy >= max_y.1 {
          true
        } else {
          false
        }
      } else {
        false
      }
    } else {
      false
    }
  }
  fn clone_piece(&self) -> Box<dyn Piece> {
    Box::new(self.clone())
  }
}
