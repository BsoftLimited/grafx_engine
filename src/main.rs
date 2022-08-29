mod grafx;

use crate::grafx::objects::models::shape::Shape;
use crate::grafx::ModelRenderer;
use crate::grafx::objects::models::model::Model;
use crate::grafx::objects::Renderable;
use crate::grafx::light::Light;
use crate::grafx::light::PointLight;
use crate::grafx::materials::BasicMaterial;
use crate::grafx::Disposable;
use crate::grafx::{Renderer, Camera, PerspectiveCamera};
use grafx::{ WindowHandler, WindowDetails};

struct Test{renderer:Box<ModelRenderer>, model:Box<Model>}

impl Test{
    unsafe fn new()->Self{
        let mat = Box::new(BasicMaterial::new());

        let model = Model::new(Shape::Box(), mat);

        let mut camera = PerspectiveCamera::new(45.0, 800, 480);
        camera.set_position(0.0, 2.0, -6.0);
        camera.update();
        
        let mut renderer = ModelRenderer::new(camera);
        let mut light = PointLight::new();
        light.set_position(0.0, 1.0, -3.0);
        light.set_intensity(5.0);

        renderer.add_point_light(light);
        Test{ renderer: Box::new(renderer), model: Box::new(model)}
       
    }
}

impl WindowHandler for Test {
    fn update(&mut self, delta: f32){
        self.model.get_transform().rotate(30.0 * delta, 30.0 * delta, 30.0 * delta);
        self.model.get_transform().update();
    }

    unsafe fn render(&self) {
        self.model.render(&self.renderer);
    }

    fn resize(&mut self, width: i32, height: i32){
        println!("new size width:{w}, height:{h}", w = width, h = height);
        self.renderer.get_camera().get_frustum_mut().set_viewport_size(width as f32, height as f32);
    }
}

impl Disposable for Test {
    fn dispose(&mut self) { self.model.dispose(); }
}

pub fn main(){
    let details = WindowDetails::new("Grafx Engine", 800, 480);
    let context = grafx::init(&details);
    unsafe{  grafx::start(context, Box::new(Test::new())); }
   
}