mod shader;

use crate::grafx::physics::Color;
use crate::grafx::materials::shader::Shader;

pub struct MaterialProperty{ diffuse:Box<Color>, ambient:Box<Color>, specular:Box<Color>, shinines:f32 }
#[allow(non_snake_case)]
#[allow(dead_code)]
impl MaterialProperty{
    pub fn new()->Self{
        MaterialProperty{
            diffuse:Box::new(Color::White()), ambient:Box::new(Color::White()), specular:Box::new(Color::White()), shinines:127.0
        }
    }
    pub fn getDiffuseColor(&self)->&Box<Color>{ &self.diffuse }
    pub fn getAmbientColor(&self)->&Box<Color>{ &self.ambient }
    pub fn getSpecularColor(&self)->&Box<Color>{ &self.specular }
    pub fn getShininess(&self)->f32{ self.shinines}
}

#[allow(non_snake_case)]
pub trait Material{
    fn getShader(&self)->&Box<Shader>;
    fn getProperties(&self)->&Box<MaterialProperty>;
    unsafe fn r#use(&self){
        let init = self.getProperties();
        self.getShader().bind();
        self.getShader().setUniformColor("material.specular", &init.getSpecularColor());
		self.getShader().setUniformColor("material.diffuse", &init.getDiffuseColor());
		self.getShader().setUniformColor("material.ambient", &init.getAmbientColor());
		self.getShader().setUniformValue("material.shininess", init.getShininess());
    }

    fn setDiffuseColor(&mut self, color:Color);
    fn setAmbientColor(&mut self, color:Color);
    fn setSpecularColor(&mut self, color:Color);
    fn setShininess(&mut self, shinines:f32);
}

pub struct BasicMaterial{ shader:Box<Shader>, properties:Box<MaterialProperty>}

impl BasicMaterial{
    pub unsafe fn new()->Self{
        let shader = Shader::Simple();
        BasicMaterial{ shader:Box::new(shader), properties:Box::new(MaterialProperty::new()) }
    }
}

impl Material for BasicMaterial {
    fn getShader(&self) -> &Box<Shader> { &self.shader }
    fn getProperties(&self) -> &Box<MaterialProperty> { &self.properties }

    fn setDiffuseColor(&mut self, color:Color){ self.properties.diffuse = Box::new(color); }
    fn setAmbientColor(&mut self, color:Color){ self.properties.ambient = Box::new(color); }
    fn setSpecularColor(&mut self, color:Color){ self.properties.specular = Box::new(color); }
    fn setShininess(&mut self, shinines:f32){ self.properties.shinines = shinines; }
}