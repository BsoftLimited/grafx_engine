
use crate::grafx::ViewPort;
use crate::grafx::Camera;

#[allow(non_snake_case)]
pub struct Renderer{ camera:Box<Camera>, viewPort:Box<ViewPort>}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Renderer{
    pub fn new(camera:Camera, viewPort:ViewPort)->Self{
        Renderer{camera:Box::new(camera), viewPort:Box::new(viewPort)}
    }

    pub fn setCamera(&mut self, camera:Camera){ self.camera = Box::new(camera);}
    pub fn setViewPort(&mut self, viewPort:ViewPort){ self.viewPort = Box::new(viewPort);}
    pub fn getView(&self)->[[f32; 4]; 4]{ self.camera.getViewMatrix().getData()}
    pub fn getProjection(&self)->[[f32; 4]; 4]{ self.viewPort.getProjectionMatrix().getData()}
}