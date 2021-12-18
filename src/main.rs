mod grafx;

use grafx::{ GameWindow, GameWindowDetails};

#[allow(non_snake_case)]
struct Test;

impl Test{ fn new()->Self{ Test{}}}

impl GameWindow for Test {
    fn initialize(&self) {
        
    }

    fn update(&self, delta: f32) {
        print!("{}\n", 1.0/delta);
    }

    fn render(&self) {  }
}

pub fn main(){
    let details = GameWindowDetails::new("Grafx Engine", 800, 480);
    grafx::run(Box::new(Test::new()), &details);
}