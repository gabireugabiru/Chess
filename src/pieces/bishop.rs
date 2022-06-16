use std::{collections::HashMap, rc::Rc};

use graphics::Transformed;
use opengl_graphics::Texture;

use super::piece::{Piece, Team};
#[derive(Clone)]
pub struct Bishop {
  texture: Rc<Texture>,
  team: Team,
}
impl Bishop {
  pub fn new(
    textures: &HashMap<&str, Rc<Texture>>,
    team: Team,
  ) -> Box<Self> {
    let texture = match team {
      Team::Green => textures.get("bishop_green").unwrap().clone(),
      Team::Black => textures.get("bishop_black").unwrap().clone(),
    };
    Box::new(Self { texture, team })
  }
}

impl Piece for Bishop {
  fn clone_piece(&self) -> Box<dyn Piece> {
    Box::new(self.clone())
  }
  fn draw(
    &self,
    args: &piston::RenderArgs,
    gl: &mut opengl_graphics::GlGraphics,
    (y, x): (usize, usize),
  ) {
    gl.draw(args.viewport(), |c, gl| {
      let transform = c.transform.trans((x * 50) as f64, (y * 50) as f64);
      graphics::image(&*self.texture, transform, gl)
    })
  }
  fn valid_positions(
    &self,
    current_pos: (usize, usize),
    table: &crate::TableVec<dyn Piece>,
  ) -> Vec<(usize, usize)> {
    let (cx, cy) = current_pos;
    let mut valid_positions: Vec<(usize, usize)> = Vec::new();
    {
      let mut i = 0;
      while cx + i < 7 && cy + i < 7 {
        i += 1;
        if let Some(piece) = &table[cx + i][cy + i] {
          if piece.team() != self.team {
            valid_positions.push((cx + i, cy + i))
          }
          break;
        }
        valid_positions.push((cx + i, cy + i))
      }
    }
    {
      let mut i = 0;
      while cx - i > 0 && cy - i > 0 {
        i += 1;
        if let Some(piece) = &table[cx - i][cy - i] {
          if piece.team() != self.team {
            valid_positions.push((cx - i, cy - i))
          }
          break;
        }
        valid_positions.push((cx - i, cy - i))
      }
    }

    {
      let mut i = 0;
      while cx - i > 0 && cy + i < 7 {
        i += 1;
        if let Some(piece) = &table[cx - i][cy + i] {
          if piece.team() != self.team {
            valid_positions.push((cx - i, cy + i))
          }
          break;
        }
        valid_positions.push((cx - i, cy + i));
      }
    }

    {
      let mut i = 0;
      while cx + i < 7 && cy - i > 0 {
        i += 1;
        if let Some(piece) = &table[cx + i][cy - i] {
          if piece.team() != self.team {
            valid_positions.push((cx + i, cy - i))
          }
          break;
        }
        valid_positions.push((cx + i, cy - i));
      }
    }
    valid_positions
  }
  fn is_position_valid(
    &self,
    current_pos: (usize, usize),
    desired_pos: (usize, usize),
    table: &crate::TableVec<dyn Piece>,
  ) -> bool {
    let valid_positions = self.valid_positions(current_pos, table);

    if valid_positions.contains(&desired_pos) {
      true
    } else {
      false
    }
  }
  fn team(&self) -> Team {
    self.team
  }
}
