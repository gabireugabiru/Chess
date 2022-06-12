use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::TableVec;

pub trait Piece {
  fn draw(
    &self,
    gl: &RenderArgs,
    gl: &mut GlGraphics,
    pos: (usize, usize),
  );
  fn is_position_valid(
    &self,
    current_pos: (usize, usize),
    desired_pos: (usize, usize),
    table: &TableVec<dyn Piece>,
  ) -> bool;
  fn clone_piece(&self) -> Box<dyn Piece>;
}
