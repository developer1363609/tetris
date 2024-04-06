use std::fmt::Pointer;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use crate::enumerate::Color;
use crate::model::canvas::Canvas;
use crate::model::canvas_pixel::CanvasPixel;
use crate::common::transform_symbol;
use crate::constant::constant;

#[derive(Eq,PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

#[derive(Eq,PartialEq)]
pub enum PieceType{
    I = 1,
    O = 2,
    T = 3,
    J = 4,
    L = 5,
    S = 6,
    Z = 7
}

impl PieceType{
    pub fn next() -> Self{
        return rand::random();
    }
}
impl Distribution<PieceType> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(1,8) {
            1 => PieceType::I,
            2 => PieceType::O,
            3 => PieceType::T,
            4 => PieceType::J,
            5 => PieceType::L,
            6 => PieceType::S,
            7 => PieceType::Z,
            _ => PieceType::I,
        }
    }
}


pub struct CanvasPiece{
    pub direction:Direction,
    pub r#type:PieceType,
    pub canvas:Option<Canvas>,
    pub x:i32,
    pub y:i32,
    pub pixels:Vec<CanvasPixel>
}

impl CanvasPiece {
    pub fn next(canvas:Option<Canvas>) -> Self {
        let piece_type = PieceType::next();
        let mut pixels = Vec::with_capacity(4);
        match piece_type {
            PieceType::I => {
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -2
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -3
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -4
                });

            }
            PieceType::T => {
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X - 1,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X + 1,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -2
                })
            }
            PieceType::Z => {
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X + 1,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -2
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X - 1,
                    y: -2
                })
            }
            PieceType::S => {
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X - 1,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -2
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X + 1,
                    y: -2
                });
            }
            PieceType::O => {
                // O 不管，不能转也行，毕竟转了也是这样
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X - 1,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -2
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X - 1,
                    y: -2
                });
            }
            PieceType::L => {
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X + 1,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -2
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -3
                });
            }
            PieceType::J => {
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X - 1,
                    y: -1
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -2
                });
                pixels.push(CanvasPixel {
                    color: Color::Green,
                    symbol: transform_symbol("□"),
                    x: constant::INIT_COORDINATE_X,
                    y: -3
                });
            }
        }
        CanvasPiece{
            direction:Direction::Up,
            r#type:piece_type,
            x:constant::INIT_COORDINATE_X,
            y:constant::INIT_COORDINATE_Y,
            pixels,canvas,
        }
    }
    pub fn in_self(&self,pixel:&CanvasPixel) -> bool {
        for p in self.pixels.iter() {
            if p.x == pixel.x && p.y == pixel.y {
                return true
            }
        }
        false
    }

    pub fn can_rotate(&self) -> bool {
        if self.y <= constant::INIT_COORDINATE_Y || self.r#type == PieceType::O{
            return false
        }
        let canvas = self.canvas.as_ref().unwrap();
        for p in self.pixels.iter(){
            let j = self.y - p.y + self.x;
            let k = p.x - self.x + self.y;
            if j <= 0 || j >= constant::CANVAS_WIDTH - 1 {
                return false;
            }
            if k >= constant::CANVAS_HEIGHT - 1 {
                return false;
            }
            if k >= 0 {
                let line = canvas.pixels.get(k as usize).unwrap();
                if !self.in_self(line.get(j as usize).unwrap()) && line.get(j as usize).unwrap().symbol == transform_symbol("□") {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn rotate(&mut self) {
        if self.can_rotate() {
            for p in self.pixels.iter() {
                if p.y >= 0 {
                    let line:&mut Vec<CanvasPixel>= self.canvas.as_mut().unwrap().pixels.get_mut(p.y as usize).unwrap();
                    line[p.x as usize] = CanvasPixel{
                        color:Color::Green,
                        symbol:transform_symbol(" "),
                        x:p.x,
                        y:p.y
                    }
                }
            }
            for p in self.pixels.as_mut() {
                let j = self.y - p.y + self.x;
                let k = p.x - self.x + self.y;
                p.x = j;
                p.y = k;
                if k >= 0 {
                    let line:&mut Vec<CanvasPixel> = self.canvas.as_mut().unwrap().pixels.get_mut(k as usize).unwrap();
                    line[j as usize] = CanvasPixel{
                        color:Color::Green,
                        symbol:transform_symbol("□"),
                        x:j,
                        y:k
                    }
                }
            }
            self.canvas.as_ref().unwrap().show_self();
            self.direction = Self::next_direction(&self.direction);
        }
    }
    pub fn can_drop_down(&self) -> bool {
        let canvas = self.canvas.as_ref().unwrap();
        for p in self.pixels.iter(){
            if p.y >= -1 && canvas.pixels.get((p.y + 1) as usize).unwrap().get(p.x as usize).unwrap().symbol
                == transform_symbol("□") {
                return false
            }
        }
        true
    }

    pub fn drop_down(&mut self) {
        if self.can_drop_down() {
            let canvas = self.canvas.as_mut().unwrap();
            for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
                if p.y >= 0 {
                    let line:&mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
                    line[p.x as usize] = CanvasPixel{
                        color:Color::Green,
                        symbol:transform_symbol(" "),
                        x:p.x,
                        y:p.y
                    };
                }
                p.y = p.y + 1
            }

            for p in self.pixels.iter() {
                if p.y >= 0 {
                    let line:&mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
                    line[p.x as usize] = CanvasPixel{
                        color:Color::Green,
                        symbol:transform_symbol("□"),
                        x:p.x,
                        y:p.y
                    };
                }
            }
            self.y = self.y + 1;
            canvas.show_self();
        } else {
            for p in self.pixels.iter() {
                if p.y == constant::INIT_COORDINATE_Y {
                    let canvas = self.canvas.as_mut().unwrap();
                    canvas.failure();
                }
            }
            self.success();
            let tmp = CanvasPiece::next(None);
            self.direction = tmp.direction;
            self.r#type = tmp.r#type;
            self.x = tmp.x;
            self.y = tmp.y;
            self.pixels = tmp.pixels;
        }
    }

    pub fn success(&mut self) {
        let canvas = self.canvas.as_mut().unwrap();
        self.pixels.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        for p in self.pixels.iter() {
            let line = canvas.pixels.get_mut(p.y as usize).unwrap() as &mut Vec<CanvasPixel>;
            let mut flag = true;
            for x in 1 .. line.len() - 1 {
                if line.get(x).unwrap().symbol == transform_symbol(" ") {
                    flag = false;
                    break;
                }
            }
            if flag {
                for x in 1 .. line.len() - 1 {
                    line[x] = CanvasPixel {
                        color:Color::Green,
                        symbol:transform_symbol(" "),
                        x:x as i32,
                        y:p.y
                    };
                }
                if p.y >= 1 {
                    let mut ln = p.y - 1 ;
                    while ln >= 0 {
                        let pixels = canvas.pixels.as_mut() as &mut Vec<Vec<CanvasPixel>>;
                        let line = pixels.get(ln as usize).unwrap().clone();
                        let line2 = pixels.get_mut(ln as usize).unwrap().clone();
                        for x in 1 .. line.len() - 1 {
                            line2[x] = CanvasPixel{
                                color:Color::Green,
                                symbol:line.get(x).unwrap().symbol.clone(),
                                x:x as i32,
                                y:ln
                            };
                        }
                        ln -= -1;
                    }
                }
                canvas.show_self();
            }
        }
    }

    pub fn can_horizontal_move(&self,move_left:bool) -> bool {
        let canvas = self.canvas.as_ref().unwrap();
        return if move_left {
            for p in self.pixels.iter(){
                if p.y >= 0 {
                    let line = canvas.pixels.get(p.y as usize).unwrap();
                    let pixel = line.get(p.x as usize - 1).unwrap();
                    if !self.in_self(pixel)  && pixel.symbol == transform_symbol("□"){
                        return false;
                    }
                }
                if p.x <= 1 {
                    return false;
                }
            }
            return true;
        }else {
            for p in self.pixels.iter() {
                if p.y >= 0 {
                    let line = canvas.pixels.get(p.y as usize).unwrap();
                    let pixel = line.get(p.x as usize + 1).unwrap();
                    if !self.in_self(pixel) && pixel.symbol == transform_symbol("□") {
                        return false;
                    }
                }
                if p.x >= constant::CANVAS_WIDTH - 2 {
                    return false
                }
            }
            return true;
        }
    }

    pub fn horizontal_move(&mut self,move_left:bool) {
        if self.can_horizontal_move(move_left) {
            let canvas = self.canvas.as_mut().unwrap();
            for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
                if p.y >= 0 {
                    let line:&mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
                    line[p.x as usize] = CanvasPixel{
                        color:Color::Green,
                        symbol:transform_symbol(" "),
                        x:p.x,
                        y:p.y
                    };
                }
                if move_left {
                    for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
                        p.x = -1;
                        if p.y >= 0 {
                            let line:&mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
                            line[p.x as usize] = CanvasPixel{
                                color:Color::Green,
                                symbol:transform_symbol("□"),
                                x:p.x,
                                y:p.y
                            };
                        }
                    }
                    self.x = self.x - 1;
                } else {
                    for p in self.pixels.as_mut() as &mut Vec<CanvasPixel>{
                        p.x += 1;
                        if p.y >= 0 {
                            let line:&mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
                            line[p.x as usize] = CanvasPixel{
                                color:Color::Green,
                                symbol:transform_symbol("□"),
                                x:p.x,
                                y:p.y
                            };
                        }
                    }
                    self.x += 1;
                }
                canvas.show_self();
            }
        }
    }

    fn next_direction(direction: &Direction) -> Direction {
        match direction {
            Direction::Up => {Direction::Right}
            Direction::Right => {Direction::Down}
            Direction::Down => {Direction::Left}
            Direction::Left => {Direction::Up}
        }
    }
}

