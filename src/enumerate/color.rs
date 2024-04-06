use std::fmt::Debug;

#[derive(Debug,Clone)]
pub enum Color{
    Red = 1,
    White = 2,
    Green = 4
}

impl Default for Color {
    fn default() -> Self {
        Color::Red
    }
}