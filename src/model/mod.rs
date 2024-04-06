use std::process::exit;
use crossterm::style;
use crossterm::style::{style, Stylize};
use crate::enumerate::Color;

pub use canvas_pixel::CanvasPixel;
pub use canvas::Canvas;
pub use canvas_piece::{CanvasPiece, PieceType, Direction};
use crate::{clear, goto};

mod canvas_pixel;
mod canvas;
mod canvas_piece;
mod coordinate;

pub trait Colorable{
    fn color(&self) -> &Color;
}

impl Colorable for CanvasPixel {
    fn color(&self) -> &Color {
        &self.color
    }
}

pub trait ShowSelf{
    fn show_self(&self);
}

impl ShowSelf for CanvasPixel {
    fn show_self(&self) {
        match &self.color {
            Color::Red => {
                print!("{}", style(format!("{}", self.symbol)).with(style::Color::Red))
            }
            Color::White => {
                print!("{}", style(format!("{}", self.symbol)).with(style::Color::White))
            }
            Color::Green => {
                print!("{}", style(format!("{}", self.symbol)).with(style::Color::Green))
            }
        }
    }
}

impl ShowSelf for Canvas {
    fn show_self(&self) {
        let _ = goto(0,0);
        for i in self.pixels.iter(){
            for j in i.iter() {
                j.show_self();
            }
            println!("\r")
        }
    }
}

impl Canvas {
    pub fn failure(&self) {
        clear();
        let _= goto(0,0);
        println!("game over");
        exit(1);
    }
}