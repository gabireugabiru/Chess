use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::TableVec;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Team {
  Green,
  Black,
}

pub trait Piece {
  fn draw(
    &self,
    args: &RenderArgs,
    gl: &mut GlGraphics,
    pos: (usize, usize),
  );
  fn is_position_valid(
    &self,
    current_pos: (usize, usize),
    desired_pos: (usize, usize),
    table: &TableVec<dyn Piece>,
  ) -> bool;
  fn valid_positions(
    &self,
    current_pos: (usize, usize),
    table: &TableVec<dyn Piece>,
  ) -> Vec<(usize, usize)>;
  fn clone_piece(&self) -> Box<dyn Piece>;
  fn team(&self) -> Team;
}
