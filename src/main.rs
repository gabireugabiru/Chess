use std::{collections::HashMap, rc::Rc};

use glutin_window::GlutinWindow;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::{
  Button, EventSettings, Events, MouseCursorEvent, ReleaseEvent,
  RenderArgs, RenderEvent, UpdateEvent, WindowSettings,
};
mod pieces {
  pub mod horse;
  pub mod piece;
  pub mod tower;
}
use pieces::{horse::Horse, piece::Piece, tower::Tower};
type TableVec<T> = Vec<Vec<Option<Box<T>>>>;

pub struct Game<'a> {
  gl: GlGraphics,
  textures: HashMap<&'a str, Rc<Texture>>,
  table: TableVec<dyn Piece>,
  pos: (f64, f64),
  selected: Option<(usize, usize)>,
  is_piece_selected: bool,
}

impl Game<'_> {
  fn new(opengl: OpenGL) -> Self {
    let gl = GlGraphics::new(opengl);

    let texture = Rc::new(
      opengl_graphics::Texture::from_path(
        "assets/TowerGreen.png",
        &TextureSettings::new(),
      )
      .unwrap(),
    );
    let texture2 = Rc::new(
      opengl_graphics::Texture::from_path(
        "assets/HorseGreen.png",
        &TextureSettings::new(),
      )
      .unwrap(),
    );

    gl.has_texture_alpha(&texture);
    gl.has_texture_alpha(&texture2);

    let mut textures = HashMap::new();
    textures.insert("tower_green", texture);
    textures.insert("horse_green", texture2);
    //CREATING TABLE
    let mut table: TableVec<dyn Piece> = vec![];
    for _ in 0..8 {
      let mut inner = Vec::new();
      for _ in 0..8 {
        inner.push(None);
      }
      table.push(inner);
    }

    //INTIATE THE PIECES THE SOLDIERS THE RAINHA DA INGALTERRA
    // table[0][0] = Some(Horse::new(&textures));
    table[0][0] = Some(Tower::new(&textures));
    table[4][4] = Some(Horse::new(&textures));

    Self {
      gl,
      pos: (0., 0.),
      textures,
      table,
      is_piece_selected: false,
      selected: None,
    }
  }
  fn render(&mut self, args: &RenderArgs) {
    let square = graphics::rectangle::square(0., 0., 50.);
    let white = [1., 1., 1., 1.];
    let yellow = [0.9, 0.8, 0.1, 1.0];
    let blue = [0., 0., 0.7, 0.4];

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
    } else {
      if self.table[y][x].is_some() {
        self.is_piece_selected = true;
        self.selected = Some((y, x));
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
