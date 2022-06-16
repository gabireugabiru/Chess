use std::{collections::HashMap, rc::Rc};

use glutin_window::GlutinWindow;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::{
  Button, EventSettings, Events, MouseCursorEvent, ReleaseEvent,
  RenderArgs, RenderEvent, UpdateEvent, WindowSettings,
};
mod pieces {
  pub mod bishop;
  pub mod horse;
  pub mod piece;
  pub mod tower;
}
use pieces::{
  bishop::Bishop,
  horse::Horse,
  piece::{Piece, Team},
  tower::Tower,
};
type TableVec<T> = Vec<Vec<Option<Box<T>>>>;
pub struct Game<'a> {
  gl: GlGraphics,
  textures: HashMap<&'a str, Rc<Texture>>,
  table: TableVec<dyn Piece>,
  pos: (f64, f64),
  selected: Option<(usize, usize)>,
  is_piece_selected: bool,
  valid_positions: Option<Vec<(usize, usize)>>,
}

impl Game<'_> {
  fn new(opengl: OpenGL) -> Self {
    let gl = GlGraphics::new(opengl);

    //CREATING TABLE
    let mut table: TableVec<dyn Piece> = vec![];
    for _ in 0..8 {
      let mut inner = Vec::new();
      for _ in 0..8 {
        inner.push(None);
      }
      table.push(inner);
    }

    let textures = HashMap::new();
    Self {
      gl,
      pos: (0., 0.),
      textures,
      table,
      is_piece_selected: false,
      selected: None,
      valid_positions: None,
    }
  }
  fn init_textures(&mut self) {
    let textures = &mut self.textures;

    textures
      .insert("horse_green", Self::texture_from_file("HorseGreen.png"));
    textures
      .insert("tower_green", Self::texture_from_file("TowerGreen.png"));
    textures
      .insert("bishop_green", Self::texture_from_file("BishopGreen.png"));

    textures
      .insert("horse_black", Self::texture_from_file("HorseBlack.png"));
    textures
      .insert("tower_black", Self::texture_from_file("TowerBlack.png"));
    textures
      .insert("bishop_black", Self::texture_from_file("BishopBlack.png"));
  }

  fn init_pieces(&mut self) {
    let table = &mut self.table;
    let textures = &self.textures;

    //INTIATE THE PIECES THE SOLDIERS THE RAINHA DA INGALTERRA
    table[3][3] = Some(Bishop::new(&textures, Team::Green));
    table[0][0] = Some(Tower::new(&textures, Team::Black));
    table[4][4] = Some(Horse::new(&textures, Team::Green));
  }

  fn texture_from_file(file: &str) -> Rc<Texture> {
    Rc::new(
      opengl_graphics::Texture::from_path(
        format!("assets/{}", file),
        &TextureSettings::new(),
      )
      .unwrap(),
    )
  }
  fn render(&mut self, args: &RenderArgs) {
    let square = graphics::rectangle::square(0., 0., 50.);
    let white = [1., 1., 1., 1.];
    let yellow = [0.9, 0.8, 0.1, 1.0];
    let blue = [0., 0., 0.7, 0.4];
    let red = [0.7, 0., 0., 0.4];

    for y in 0..8 {
      for x in 0..8 {
        let shifted = if y % 2 == 0 { x + 1 } else { x };
        let color = if shifted % 2 == 0 { yellow } else { white };

        self.gl.draw(args.viewport(), |c, gl| {
          let transform =
            c.transform.trans((y * 50) as f64, (x * 50) as f64);
          graphics::rectangle(color, square, transform, gl);
        });
        if let Some(piece) = &self.table[x][y] {
          piece.draw(&args, &mut self.gl, (x, y));
        }
      }
    }
    if let Some(pos) = self.selected {
      if self.table[pos.0][pos.1].is_some() {
        self.gl.draw(args.viewport(), |c, gl| {
          let transform =
            c.transform.trans((pos.1 * 50) as f64, (pos.0 * 50) as f64);
          graphics::rectangle(blue, square, transform, gl);
        })
      }
    }
    if let Some(valid_pos) = &self.valid_positions {
      for pos in valid_pos {
        self.gl.draw(args.viewport(), |c, gl| {
          let transform =
            c.transform.trans((pos.1 * 50) as f64, (pos.0 * 50) as f64);
          graphics::rectangle(red, square, transform, gl);
        });
      }
    }
  }
  fn set_pos(&mut self, pos: (f64, f64)) {
    self.pos = pos;
  }

  fn click(&mut self) {
    if self.pos.0 < 0.
      || self.pos.0 > 400.
      || self.pos.1 < 0.
      || self.pos.1 > 400.
    {
      println!("outside accepted range");
      return;
    }
    let x = (self.pos.0 / 50.) as usize;
    let y = (self.pos.1 / 50.) as usize;

    // IN THE FUTURE CHECK THE TEAM OF THE PIECE
    if self.is_piece_selected {
      let pos = self.selected.unwrap();
      if let Some(piece) = &self.table[pos.0][pos.1] {
        println!("CURRENT POS: {:?}\nDESIRED POS: {:?}", pos, (y, x));
        if piece.is_position_valid(pos, (y, x), &self.table) {
          self.table[y][x] = Some(piece.clone_piece());
          self.table[pos.0][pos.1] = None;
        }
      }
      self.is_piece_selected = false;
      self.selected = None;
      self.valid_positions = None;
    } else {
      if let Some(piece) = &self.table[y][x] {
        self.is_piece_selected = true;
        self.selected = Some((y, x));
        self.valid_positions =
          Some(piece.valid_positions((y, x), &self.table))
      }
    }
  }
}

fn main() {
  let opengl = OpenGL::V3_2;

  let mut window: GlutinWindow =
    WindowSettings::new("Chess 2", [400, 400])
      .graphics_api(opengl)
      .build()
      .unwrap();
  let mut game = Game::new(opengl);
  game.init_textures();
  game.init_pieces();

  let mut events = Events::new(EventSettings::new());

  while let Some(ev) = events.next(&mut window) {
    if let Some(args) = ev.render_args() {
      game.render(&args);
    }
    if let Some(_) = ev.update_args() {
      // game.update()
    }

    if let Some(input) = ev.release_args() {
      match input {
        Button::Mouse(_) => game.click(),
        _ => {}
      }
    }

    ev.mouse_cursor(|t| game.set_pos((t[0], t[1])));
  }
}
