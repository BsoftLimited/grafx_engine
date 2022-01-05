use crate::grafx::physics::{Vector3, Matrix4};

pub enum ViewPortType{ Perspective(f32, f32, f32), Orthographic }

#[allow(non_snake_case)]
pub struct ViewPort{ projection:Matrix4, portType:ViewPortType,  viewPointWidth:i32, viewPointHeight:i32}
#[allow(non_snake_case)]
#[allow(dead_code)]
impl ViewPort{
    pub fn Perspective(fov:f32, width:i32, height:i32)->Self{
        let view = Matrix4::ProjectionMatrix(fov, width as f32, height as f32, 1.0, 1000.0);
        ViewPort{ projection:view, portType:ViewPortType::Perspective(fov, 1.0, 1000.0), viewPointWidth:width, viewPointHeight:height}
    }

    pub fn getViewPortWidth(&self)->i32{ self.viewPointWidth}
    pub fn getViewPortHeiht(&self)->i32{ self.viewPointHeight}
    pub fn getType(&self)->&ViewPortType{ &self.portType }

    pub fn setFOV(&mut self, fov:f32){
        if let ViewPortType::Perspective(field, _, _) = &mut self.portType {
            *field = fov;
            self.projection = Matrix4::ProjectionMatrix(fov, self.viewPointWidth as f32, self.viewPointHeight as f32, 1.0, 1000.0);
        }
    }

    pub fn setViewPortSize(&mut self, width:i32, hieght:i32){
        self.viewPointWidth = width;
        self.viewPointHeight = hieght;
        if let ViewPortType::Perspective(fov, near, far) = self.portType {
            self.projection = Matrix4::ProjectionMatrix(fov, self.viewPointWidth as f32, self.viewPointHeight as f32, near, far);
        }
    }

    pub fn getProjectionMatrix(&self)->&Matrix4{ &self.projection }
}

#[allow(non_snake_case)]
pub struct Camera{  view:Matrix4, position:Vector3, target:Vector3, }
#[allow(non_snake_case)]
#[allow(dead_code)]
impl Camera{
    pub fn new(x:f32, y:f32, z:f32)->Self{
        let position = Vector3::new(x, y, z);
        let target = Vector3::zero();
        let view = Matrix4::LookAtMatrix(&position, &target, &Vector3::up());
        Camera{view, position, target}
    }

    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){
        self.position.set(x, y, z);
        self.view = Matrix4::LookAtMatrix(&self.position, &self.target, &Vector3::up());
    }

    pub fn translate(&mut self, x:f32, y:f32, z:f32){
        self.position.addV(x, y, z);
        self.view = Matrix4::LookAtMatrix(&self.position, &self.target, &Vector3::up());
    }

    pub fn getViewMatrix(&self)->&Matrix4{ &self.view }
}