mod grafx;

use crate::grafx::{Renderer, ViewPort, Camera, Shader, models::{shape::Shape, model::Model}};
use grafx::{ GameWindow, GameWindowDetails};

#[allow(non_snake_case)]
struct TestObjects{ renderer:Box<Renderer>, model:Box<Model>}

#[allow(non_snake_case)]
struct Test{component:Option<TestObjects>}

impl Test{ fn new()->Self{ Test{component:None}}}

impl GameWindow for Test {
    fn initialize(&mut self) {
        unsafe{
            let model = Model::new(Shape::Box(), Shader::Simple());
            let renderer = Renderer::new(Camera::new(5.0, 5.0, 5.0), ViewPort::new(45.0, 800, 480));
            self.component = std::option::Option::Some(TestObjects{renderer:Box::new(renderer), model:Box::new(model)})
        }
    }

    fn update(&mut self, delta: f32){
        match &mut self.component{
            Some(value) =>{
                value.model.getTransform().rotate(30.0 * delta, 30.0 * delta, 30.0 * delta);
                value.model.getTransform().update();
            }
            None =>()
        }
    }

    unsafe fn render(&self) {
        match &self.component{
            Some(value) =>{
                value.model.render(&value.renderer);
            }
            None =>()
        }
    }
}

pub fn main(){
    let details = GameWindowDetails::new("Grafx Engine", 800, 480);
    grafx::run(Box::new(Test::new()), &details);
}