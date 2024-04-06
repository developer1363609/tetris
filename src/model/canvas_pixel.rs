use crate::enumerate::Color;
#[derive(Default,Debug,Clone)]
pub struct CanvasPixel{
    pub color:Color,
    pub symbol:String,
    pub x:i32,
    pub y:i32
}