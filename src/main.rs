use std::io::{stdout, Write};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossterm::cursor::MoveTo;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::execute;
use crossterm::style::{ Stylize};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use lazy_static::lazy_static;
use crate::common::transform_symbol;
use crate::enumerate::Color;

mod constant;
mod enumerate;
mod model;
mod common;


use crate::model::{ShowSelf, Canvas, CanvasPixel, CanvasPiece, PieceType, Direction};


const HELP: &str = r#"Press Ctrl+Q to end, press c then u to start the game：
"#;

static mut CURR_PIECE:CanvasPiece = CanvasPiece{
    r#type:PieceType::I,
    direction:Direction::Up,
    x:constant::constant::INIT_COORDINATE_X,
    y:constant::constant::INIT_COORDINATE_Y,
    canvas:None,
    pixels:Vec::new()
};

lazy_static!{
    static ref CURR_PIECE_LOCK:Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

fn print_events() -> crossterm<()> {
    let mut canvas = Canvas{
        pixels:Vec::with_capacity(22),
    };
    for i in 0 .. canvas.pixels.capacity() {
        let mut y = Vec::with_capacity(12) as Vec<CanvasPixel>;
        for j in 0 .. y.capacity() {
            if i == canvas.pixels.capacity() - 1 {
                y.push(CanvasPixel{
                    color:Color::Red,
                    symbol:transform_symbol("□"),
                    x:j as i32,
                    y:i as i32
                });
            } else if j == 0 || j == y.capacity() - 1 {
                y.push(CanvasPixel{
                    color:Color::Red,
                    symbol:transform_symbol("□"),
                    x:j as i32,
                    y:i as i32
                })
            } else {
                y.push(CanvasPixel{
                    color:Color::Green,
                    symbol:transform_symbol(" "),
                    x:j as i32,
                    y:i as i32
                })
            }
        }
        canvas.pixels.push(y);
    }
    unsafe {
        CURR_PIECE = CanvasPiece::next(Some(canvas));
    }
    let canvas:&Canvas;
    unsafe {
        canvas = CURR_PIECE.as_ref().unwrap();
    }
    let mut game_started = false;
    let lock_done = CURR_PIECE_LOCK.clone();
    loop {
        let event = read()?;
        if event == Event::Key(KeyCode::Char('c').into()) {
            clear();
            let _ = goto(0,0);
        }

        if event == Event::Key(KeyCode::Char('p').into()) {
            println!("{}", "□〠".len());
        }

        if event == Event::Key(KeyCode::Char('f').into()) {
            println!("{}", concat!("red foreground color\r", "***concat***").red());
            println!("{}", "white foreground color\r".white());
            println!("{}", "green background color\r".on_green());
            println!("{}", "white background color\r".on_white());
        }

        if event == Event::Key(KeyCode::Char('m').into()) {
            let _ = goto(0,0);
        }

        if event == Event::Key(KeyCode::Char('y').into()) {
            println!("{}", "▉▉▉▉▓▓▓▓▓▓██▉▉☾☽██◀▶██████╲╱██◑◐██ღღ88");
            println!("{}", "◇◆●○■□██◁▶▉▉██╳╳><╱╲██♪♩▓▉88");
        }

        if event == Event::Key(KeyCode::Char('z').into()) {
            unsafe {
                let string = format!("{}{}","\n",CURR_PIECE.y);
                println!("{}",transform_symbol(string.deref()));
            }
            flush_output();
        }

        if event == Event::Key(KeyCode::Up.into()) {
            unsafe {
                CURR_PIECE.rotate();
            }
            print!("{}",transform_symbol("\r\n"));
            flush_output();
        }

        if event == Event::Key(KeyCode::Right.into()) {
            unsafe {
                CURR_PIECE.horizontal_move(false);
            }
            print!("{}", transform_symbol("\r→"));
            flush_output();
        }

        if event == Event::Key(KeyCode::Down.into()) {
            unsafe {
                CURR_PIECE.drop_down()
            }
            print!("{}", transform_symbol("\r↓"));
            flush_output();
        }

        if event == Event::Key(KeyCode::Left.into()) {
            unsafe {
                // 不用unsafe会提示curr_piece可能被并发修改，但是我确实只用了一个线程因此unsafe【还有个问题，为什么用drop方法会提示必须用mut定义？】
                CURR_PIECE.horizontal_move(true);
            }
            print!("{}", transform_symbol("\r←"));
            flush_output();
        }
        
        if event == Event::Key(KeyCode::Char('u').into()) {
            if game_started {
                continue;
            }
            game_started = false;
            canvas.show_self();
            let lock_clone = CURR_PIECE_LOCK.clone();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_millis(1500));
                    unsafe {
                        CURR_PIECE.drop_down();
                    }
                }
            });
        }
        if event == Event::Key(KeyCode::Char('e').into()) {
            let piece = CanvasPixel{
                color:Color::Red,
                symbol:"*".to_owned(),
                x:5,
                y:5
            };
            piece.show_self();
            flush_output();
        }
        if event == Event::Key(KeyEvent::new(KeyCode::Char('q'),KeyModifiers::CONTROL)){
            println!("game quit!\r");
            break;
        }
    }
    Ok(()).expect("TODO: panic message");
}

fn flush_output() {
    let _ = stdout().flush();
}

fn main() {
    println!("{}", HELP);
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout,EnableMouseCapture)?;
    if let Err(e) = print_events(){
        println!("Error : {:?}\r",e)
    }
    execute!(stdout,DisableMouseCapture)?;
    disable_raw_mode().expect("TODO: panic message");
}


fn goto(x: u16, y: u16) -> Result<()> {
    execute!(stdout(), MoveTo(x, y))?;
    Ok(())
}

fn clear() {
    let _ = execute!(stdout(),Clear(ClearType::All));
}