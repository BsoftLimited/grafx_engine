use crate::grafx::models::shape::Shape;
use crate::grafx::Shader;
use crate::grafx::Renderer;
use crate::grafx::physics::Transformation;

#[allow(non_snake_case)]
#[allow(dead_code)]
struct Model{ transform:Box<Transformation>, shape: Box<Shape>, shader:Box<Shader>}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Model{
    pub fn new( shape: Shape, shader:Shader)->Self{
        Model{ transform:Box::new(Transformation::new()), shape:Box::new(shape), shader:Box::new(shader)}
    }

    pub fn getTransform(&mut self)->&Box<Transformation>{ &self.transform }
    pub fn setShader(&mut self, shader:Shader){ self.shader = Box::new(shader); }
    pub fn setShape(&mut self, shape:Shape){ self.shape = Box::new(shape); }

    pub fn render(&self, renderer:&Renderer){
        unsafe{
            gl::BindVertexArray(self.shape.getVertexArrayBuffer());
            self.shader.bind();
            self.shader.setUniformMatrix("transform", self.transform.getTransform().getData());
            self.shader.setUniformMatrix("projection", renderer.getProjection());
            self.shader.setUniformMatrix("view", renderer.getView());
            gl::DrawElements(gl::TRIANGLES, self.shape.getIndexCount(), gl::UNSIGNED_INT,  0 as *const _);
        }
    }
}