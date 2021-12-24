use crate::grafx::materials::Material;
use crate::grafx::models::shape::Shape;
use crate::grafx::Renderer;
use crate::grafx::physics::Transformation;
use crate::grafx::light::LightType;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Model{ transform:Box<Transformation>, shape: Box<Shape>, material:Box<dyn Material>}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Model{
    pub fn new( shape: Shape, material:Box<dyn Material>)->Self{
        Model{ transform:Box::new(Transformation::new()), shape:Box::new(shape), material}
    }

    pub fn getTransform(&mut self)->&mut Box<Transformation>{ &mut self.transform }
    pub fn setMaterial(&mut self, material:Box< dyn Material>){ self.material = material; }
    pub fn setShape(&mut self, shape:Shape){ self.shape = Box::new(shape); }

    pub fn render(&self, renderer:&Box<Renderer>){
        unsafe{
            gl::BindVertexArray(self.shape.getVertexArrayBuffer());
            self.material.r#use();
            self.material.getShader().setUniformMatrix("transform", self.transform.getTransformMatrix());
            self.material.getShader().setUniformMatrix("projection", renderer.getProjection());
            self.material.getShader().setUniformMatrix("view", renderer.getView());
            let lights = renderer.getLights();
            if lights.len() > 0{
                let mut nDirectional:i32 = 0;
                let mut nSpot:i32 = 0;
                let mut nPoint:i32 = 0;
                for light in lights {
                    match light.getType(){
                        LightType::DirectionalLight(direction)=>{
                            self.material.getShader().setUniformColor(format!("dirLights[{}].color", nDirectional).as_ref(), light.getColor());
                            self.material.getShader().setUniformVector3(format!("dirLights[{}].direction", nDirectional).as_ref(), direction.getDirection());
		                    self.material.getShader().setUniformValue(format!("dirLights[{}].intensity", nDirectional).as_ref(), light.getIntensity());
                            self.material.getShader().setUniformValue(format!("dirLights[{}].ambientStrenght", nDirectional).as_ref(),  light.getAmbientStrenght());
                            nDirectional += 1;
                        },
                        LightType::SpotLight(spot) =>{
                            self.material.getShader().setUniformVector3(format!("spotLights[{}].position", nSpot).as_ref(), spot.getPosition());
                            self.material.getShader().setUniformColor(format!("spotLights[{}].color", nSpot).as_ref(), light.getColor());
                            self.material.getShader().setUniformValue(format!("spotLights[{}].intensity", nSpot).as_ref(), light.getIntensity());
                            self.material.getShader().setUniformVector3(format!("spotLights[{}].direction", nSpot).as_ref(), spot.getDirection());
                            
                            self.material.getShader().setUniformValue(format!("spotLights[{}].linear", nSpot).as_ref(), spot.getLinear());
                            self.material.getShader().setUniformValue(format!("spotLights[{}].quadratic", nSpot).as_ref(), spot.getQuadratic());
                            self.material.getShader().setUniformValue(format!("spotLights[{}].radius", nSpot).as_ref(), spot.getInnerRadius());
                            self.material.getShader().setUniformValue(format!("spotLights[{}].outerRadius", nSpot).as_ref(), spot.getOuterRadius());
                            self.material.getShader().setUniformValue(format!("spotLights[{}].ambientStrenght", nSpot).as_ref(),  light.getAmbientStrenght());
                            nSpot += 1;
                        },
                        LightType::PointLight(point) => {
                            self.material.getShader().setUniformColor(format!("pointLights[{}].color", nPoint).as_ref(), light.getColor());
                            self.material.getShader().setUniformVector3(format!("pointLights[{}].position", nPoint).as_ref(), point.getPosition());
                            self.material.getShader().setUniformValue(format!("pointLights[{}].intensity", nPoint).as_ref(), light.getIntensity());
                            self.material.getShader().setUniformValue(format!("pointLights[{}].linear", nPoint).as_ref(), point.getLinear());
                            self.material.getShader().setUniformValue(format!("pointLights[{}].quadratic", nPoint).as_ref(), point.getQuadratic());
                            self.material.getShader().setUniformValue(format!("pointLights[{}].ambientStrenght", nPoint).as_ref(),  light.getAmbientStrenght());
                            nPoint += 1;
                        }
                    }
                }
                self.material.getShader().setUniformInt("nDir", nDirectional);
				self.material.getShader().setUniformInt("nPoint", nPoint);
				self.material.getShader().setUniformInt("nSpot", nSpot);
            }
            gl::DrawElements(gl::TRIANGLES, self.shape.getIndexCount(), gl::UNSIGNED_INT,  0 as *const _);
        }
    }
}