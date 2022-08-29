use crate::grafx::light::LightType;
use crate::grafx::light::SpotLight;
use crate::grafx::light::PointLight;
use crate::grafx::light::Vector3;
use crate::grafx::light::Light;
use crate::grafx::light::Color;

pub struct DirectionalLight{ id:String, color:Box<Color>, intensity:f32, ambient_strenght:f32, direction:Box<Vector3> }
impl DirectionalLight{
    pub fn new()->Self{
        let mut n = nuid::NUID::new();
        DirectionalLight{id:n.next(), color:Box::new(Color::white()), intensity:1.0, ambient_strenght: 0.2, direction:Box::new(Vector3::zero())}
    }
    pub fn set_direction(&mut self, x:f32, y:f32, z:f32){ self.direction = Box::new(Vector3::new(x, y, z)); }
    pub fn get_direction(&self)->&Vector3{ self.direction.as_ref() }
    pub fn get_direction_mut(&mut self)->&mut Vector3{ self.direction.as_mut() }
}

impl Light for DirectionalLight{
    fn get_id(&self)->&str{ self.id.as_ref() }
    fn get_color(&self)->&Color{ self.color.as_ref() }
    fn set_color(&mut self, color: Color){ self.color = Box::new(color); }
    fn set_intensity(&mut self, intensity:f32){ self.intensity = intensity; }
    fn get_intensity(&self)->f32{ self.intensity }
    fn get_ambient_strenght(&self)->f32{ self.ambient_strenght}
    fn set_ambient_strenght(&mut self, ambient_strenght:f32){ self.ambient_strenght = ambient_strenght}
    fn get_type(&self)->LightType{ LightType::DirectionalLight }

    fn as_directional(&self)->Option<&DirectionalLight>{ Some(self)}
    fn as_spot(&self)->Option<&SpotLight>{ None }
    fn as_point(&self)->Option<&PointLight>{ None }

    fn as_directional_mut(&mut self)->Option<&mut DirectionalLight>{ Some(self) }
    fn as_spot_mut(&mut self)->Option<&mut SpotLight>{ None }
    fn as_point_mut(&mut self)->Option<&mut PointLight>{ None }
}