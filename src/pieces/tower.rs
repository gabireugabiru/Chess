use std::{collections::HashMap, rc::Rc};

use graphics::Transformed;
use opengl_graphics::{GlGraphics, Texture};
use piston::RenderArgs;

use crate::{Piece, TableVec};

use super::piece::Team;
#[derive(Clone)]
pub struct Tower {
  texture: Rc<Texture>,
  team: Team,
}

impl Tower {
  pub fn new(
    textures: &HashMap<&str, Rc<Texture>>,
    team: Team,
  ) -> Box<Self> {
    let texture = match team {
      Team::Black => textures.get("tower_black").unwrap().clone(),
      Team::Green => textures.get("tower_green").unwrap().clone(),
    };
    Box::new(Self { texture, team })
  }
}
impl Piece for Tower {
  fn draw(
    &self,
    args: &RenderArgs,
    gl: &mut GlGraphics,
    (y, x): (usize, usize),
  ) {
    gl.draw(args.viewport(), |c, gl| {
      let transformers_o_segundo_filme =
        c.transform.trans((x * 50) as f64, (y * 50) as f64);
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
    let (cx, cy) = current_pos;

    let mut valid_positions: Vec<(usize, usize)> = Vec::new();

    for i in cx + 1..8 {
      if let Some(piece) = &table[i][cy] {
        if piece.team() != self.team {
          valid_positions.push((i, cy));
        }
        break;
      }
      valid_positions.push((i, cy));
    }
    if cx != 0 {
      let mut i = cx;
      while i > 0 {
        i -= 1;
        if let Some(piece) = &table[i][cy] {
          if piece.team() != self.team {
            valid_positions.push((i, cy));
          }
          break;
        }
        valid_positions.push((i, cy));
      }
    }

    for i in cy + 1..8 {
      if let Some(piece) = &table[cx][i] {
        if piece.team() != self.team {
          valid_positions.push((cx, i));
        }
        break;
      }
      valid_positions.push((cx, i));
    }
    if cy != 0 {
      let mut i = cy;
      while i > 0 {
        i -= 1;
        if let Some(piece) = &table[cx][i] {
          if piece.team() != self.team {
            valid_positions.push((cx, i));
          }
          break;
        }
        valid_positions.push((cx, i));
      }
    }

    if valid_positions.contains(&desired_pos) {
      true
    } else {
      false
    }
  }
  fn clone_piece(&self) -> Box<dyn Piece> {
    Box::new(self.clone())
  }
  fn team(&self) -> Team {
    self.team
  }
}
