
use crate::grafx::light::Light;
use crate::grafx::ViewPort;
use crate::grafx::Camera;

#[allow(non_snake_case)]
pub struct Renderer{ camera:Box<Camera>, viewPort:Box<ViewPort>, lights:Vec<Box<dyn Light>>}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Renderer{
    pub fn new(camera:Camera, viewPort:ViewPort)->Self{
        Renderer{camera:Box::new(camera), viewPort:Box::new(viewPort), lights:Vec::new()}
    }

    pub fn setCamera(&mut self, camera:Camera){ self.camera = Box::new(camera);}
    pub fn setViewPort(&mut self, viewPort:ViewPort){ self.viewPort = Box::new(viewPort);}
    pub fn getView(&self)->[[f32; 4]; 4]{ self.camera.getViewMatrix().getData()}
    pub fn getProjection(&self)->[[f32; 4]; 4]{ self.viewPort.getProjectionMatrix().getData()}
    pub fn getLights(&self)->&Vec<Box<dyn Light>>{ &self.lights }

    pub fn addLight(&mut self, light:Box<dyn Light>)->bool{
        let init = &light;
        for n in 0..self.lights.len(){
            if self.lights[n].getID() == init.getID(){ return false; }
        }
        self.lights.push(light);
        return true;
    }

    pub fn removeLight<T:Light>(&mut self, light:&T)->bool{
        for n in 0..self.lights.len(){
            if self.lights[n].getID() == light.getID(){
                self.lights.remove(n);
                return true;
            }
        }
        return false;
    }

    pub fn getLightByID(&self, cameraID:&str)->Option<&Box<dyn Light>>{
        for n in 0..self.lights.len(){
            if self.lights[n].getID() == cameraID{
                return Some(&self.lights[n])
            }
        }
        None
    }


    pub fn getLightByIndex(&self, index:usize)->Option<&Box<dyn Light>>{
        if index < self.lights.len(){
            return Some(&self.lights[index])
        }
        return None;
    }
}