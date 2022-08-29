use crate::grafx::objects::models::shape::Shape;
use crate::grafx::renderer::ModelRenderer;
use crate::grafx::light::Light;
use crate::grafx::objects::Renderable;
use crate::grafx::objects::GameObject3D;
use crate::Disposable;
use crate::grafx::materials::Material;
use crate::grafx::Renderer;
use crate::grafx::physics::Transformation3D;
use crate::grafx::light::LightType;

#[allow(dead_code)]
pub struct Model{ transform:Box<Transformation3D>, shape: Box<Shape>, material:Box<dyn Material>}


#[allow(dead_code)]
impl Model{
    pub fn new( shape: Shape, material:Box<dyn Material>)->Self{
        Model{ transform:Box::new(Transformation3D::new()), shape:Box::new(shape), material}
    }

    pub fn get_transform(&mut self)->&mut Transformation3D{ self.transform.as_mut() }
    pub fn set_material(&mut self, material:Box< dyn Material>){ self.material = material; }
    pub fn set_shape(&mut self, shape:Shape){ self.shape = Box::new(shape); }
}

impl Renderable<ModelRenderer> for Model{
    fn render(&self, renderer:&ModelRenderer){
        unsafe{
            gl::BindVertexArray(self.shape.getVertexArrayBuffer());
            self.material.r#use();
            self.material.get_shader().set_uniform_matrix4("transform", self.transform.get_transform_matrix());
            self.material.get_shader().set_uniform_matrix4("projection", renderer.get_projection());
            self.material.get_shader().set_uniform_matrix4("view", renderer.get_view());
            let lights = renderer.get_lights();
            if lights.len() > 0{
                let mut n_directional:i32 = 0;
                let mut n_spot:i32 = 0;
                let mut n_point:i32 = 0;
                for plight in lights {
                    match plight.1.get_type(){
                        LightType::DirectionalLight =>{
                            let light = plight.1.as_ref().as_directional().unwrap();
                            self.material.get_shader().set_uniform_color(format!("dirLights[{}].color", n_directional).as_ref(), light.get_color());
                            self.material.get_shader().set_uniform_vector3(format!("dirLights[{}].direction", n_directional).as_ref(), light.get_direction());
		                    self.material.get_shader().set_uniform_value(format!("dirLights[{}].intensity", n_directional).as_ref(), light.get_intensity());
                            self.material.get_shader().set_uniform_value(format!("dirLights[{}].ambientStrenght", n_directional).as_ref(),  light.get_ambient_strenght());
                            n_directional += 1;
                        },
                        LightType::SpotLight =>{
                            let light = plight.1.as_ref().as_spot().unwrap();
                            self.material.get_shader().set_uniform_vector3(format!("spotLights[{}].position", n_spot).as_ref(), light.get_position());
                            self.material.get_shader().set_uniform_color(format!("spotLights[{}].color", n_spot).as_ref(), light.get_color());
                            self.material.get_shader().set_uniform_value(format!("spotLights[{}].intensity", n_spot).as_ref(), light.get_intensity());
                            self.material.get_shader().set_uniform_vector3(format!("spotLights[{}].direction", n_spot).as_ref(), light.get_direction());
                            
                            self.material.get_shader().set_uniform_value(format!("spotLights[{}].linear", n_spot).as_ref(), light.get_linear());
                            self.material.get_shader().set_uniform_value(format!("spotLights[{}].quadratic", n_spot).as_ref(), light.get_quadratic());
                            self.material.get_shader().set_uniform_value(format!("spotLights[{}].radius", n_spot).as_ref(), light.get_inner_radius());
                            self.material.get_shader().set_uniform_value(format!("spotLights[{}].outerRadius", n_spot).as_ref(), light.get_outer_radius());
                            self.material.get_shader().set_uniform_value(format!("spotLights[{}].ambientStrenght", n_spot).as_ref(),  light.get_ambient_strenght());
                            n_spot += 1;
                        },
                        LightType::PointLight => {
                            let light = plight.1.as_ref().as_point().unwrap();
                            self.material.get_shader().set_uniform_color(format!("pointLights[{}].color", n_point).as_ref(), light.get_color());
                            self.material.get_shader().set_uniform_vector3(format!("pointLights[{}].position", n_point).as_ref(), light.get_position());
                            self.material.get_shader().set_uniform_value(format!("pointLights[{}].intensity", n_point).as_ref(), light.get_intensity());
                            self.material.get_shader().set_uniform_value(format!("pointLights[{}].linear", n_point).as_ref(), light.get_linear());
                            self.material.get_shader().set_uniform_value(format!("pointLights[{}].quadratic", n_point).as_ref(), light.get_quadratic());
                            self.material.get_shader().set_uniform_value(format!("pointLights[{}].ambientStrenght", n_point).as_ref(),  light.get_ambient_strenght());
                            n_point += 1;
                        }
                    }
                }
                self.material.get_shader().set_uniform_int("nDir", n_directional);
				self.material.get_shader().set_uniform_int("nPoint", n_point);
				self.material.get_shader().set_uniform_int("nSpot", n_spot);
            }
            gl::DrawElements(gl::TRIANGLES, self.shape.getIndexCount(), gl::UNSIGNED_INT,  0 as *const _);
        }
    }
}

impl GameObject3D for Model{
    /**/
}

impl Disposable for Model{
    fn dispose(&mut self) { self.material.dispose(); }
}