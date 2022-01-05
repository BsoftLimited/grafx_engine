mod grafx;

use crate::grafx::materials::BasicMaterial;
use crate::grafx::Disposable;
use crate::grafx::{LightType, Light};
use crate::grafx::{Renderer, ViewPort, Camera, models::{shape::Shape, model::Model}};
use grafx::{ WindowHandler, WindowDetails};


#[allow(non_snake_case)]
struct Test{renderer:Box<Renderer>, model:Box<Model>}

impl Test{
    unsafe fn new()->Self{
        let mat = Box::new(BasicMaterial::new());

        let model = Model::new(Shape::Box(), mat);
        let mut renderer = Renderer::new(Camera::new(0.0, 2.0, -6.0), ViewPort::Perspective(45.0, 800, 480));
        let mut light = Box::new(Light::new(LightType::newPoint()));
        light.setPosition(0.0, 1.0, -3.0);
        light.setIntensity(3.0);

        renderer.addLight(light);
        Test{ renderer: Box::new(renderer), model: Box::new(model)}
    }
}

impl WindowHandler for Test {
    fn update(&mut self, delta: f32){
        self.model.getTransform().rotate(30.0 * delta, 30.0 * delta, 30.0 * delta);
        self.model.getTransform().update();
    }

    unsafe fn render(&self) {
        self.model.render(&self.renderer);
    }

    fn resize(&mut self, width: i32, height: i32){
        println!("new size width:{w}, height:{h}", w = width, h = height);
        self.renderer.getViewPort().setViewPortSize(width, height);
    }
}

impl Disposable for Test {
    fn dispose(&self) {

    }
}

pub fn main(){
    let details = WindowDetails::new("Grafx Engine", 800, 480);
    let context = grafx::init(&details);
    unsafe{  grafx::start(context, Box::new(Test::new())); }
   
}