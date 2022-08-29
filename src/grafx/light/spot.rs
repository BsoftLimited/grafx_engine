use crate::grafx::light::LightType;
use crate::grafx::light::PointLight;
use crate::grafx::light::DirectionalLight;
use crate::grafx::light::Light;
use crate::grafx::light::Vector3;
use crate::grafx::light::Color;

pub struct SpotLight{  id:String, color:Box<Color>, intensity:f32, ambient_strenght:f32, position:Box<Vector3>, direction:Box<Vector3>, max_distance:f32, radius:f32 }
#[allow(dead_code)]
impl SpotLight{
    pub fn new()->Self{
        let mut n = nuid::NUID::new();
        SpotLight{
            id:n.next(), color:Box::new(Color::white()), intensity:1.0, ambient_strenght: 0.2, 
            position:Box::new(Vector3::zero()), direction:Box::new(Vector3::zero()), max_distance: 0.7, radius:30.0 }
    }

    pub fn get_linear(&self)->f32{ return 4.5 / self.max_distance}

    pub fn get_quadratic(&self)->f32{
        return (8.5 / self.max_distance).powf(2.0);
    }

    pub fn set_position(&mut self, x:f32, y:f32, z:f32){
        self.position = Box::new(Vector3::new(x, y, z));
    }

    pub fn set_direction(&mut self, x:f32, y:f32, z:f32){
        self.direction = Box::new(Vector3::new(x, y, z))
    }

    pub fn get_position(&self)->&Vector3{ self.position.as_ref() }
    pub fn get_direction(&self)->&Vector3{ self.direction.as_ref() }
    pub fn set_max_distance(&mut self, distance:f32){ self.max_distance = distance; }

    pub fn get_radius(&self)->f32{ self.radius}
    pub fn get_inner_radius(&self)->f32{ f32::cos(f32::to_radians(self.radius/2.0))}
    pub fn get_outer_radius(&self)->f32{ f32::cos(f32::to_radians(self.radius))}

    pub fn set_radius(&mut self, radius:f32){ self.radius = radius; }
}

impl Light for SpotLight{
    fn get_id(&self)->&str{ self.id.as_ref() }
    fn get_color(&self)->&Color{ self.color.as_ref() }
    fn set_color(&mut self, color: Color){ self.color = Box::new(color); }
    fn set_intensity(&mut self, intensity:f32){ self.intensity = intensity; }
    fn get_intensity(&self)->f32{ self.intensity }
    fn get_ambient_strenght(&self)->f32{ self.ambient_strenght}
    fn set_ambient_strenght(&mut self, ambient_strenght:f32){ self.ambient_strenght = ambient_strenght}
    fn get_type(&self)->LightType{ LightType::SpotLight }

    fn as_directional(&self)->Option<&DirectionalLight>{ None }
    fn as_spot(&self)->Option<&SpotLight>{ Some(self) }
    fn as_point(&self)->Option<&PointLight>{ None }

    fn as_directional_mut(&mut self)->Option<&mut DirectionalLight>{ None }
    fn as_spot_mut(&mut self)->Option<&mut SpotLight>{ Some(self) }
    fn as_point_mut(&mut self)->Option<&mut PointLight>{ None }
}