use crate::grafx::physics::{ Color, Vector3};

mod directional;
pub use directional::DirectionalLight;

mod point;
pub use point::PointLight;

mod spot;
pub use spot::SpotLight;

pub enum LightType{ DirectionalLight, PointLight, SpotLight }

pub trait Light{
    fn get_id(&self)->&str;
    fn get_color(&self)->&Color;
    fn set_color(&mut self, color: Color);
    fn set_intensity(&mut self, intensity:f32);
    fn get_intensity(&self)->f32;
    fn get_ambient_strenght(&self)->f32;
    fn set_ambient_strenght(&mut self, ambient_strenght:f32);
    fn get_type(&self)->LightType;

    fn as_directional(&self)->Option<&DirectionalLight>;
    fn as_spot(&self)->Option<&SpotLight>;
    fn as_point(&self)->Option<&PointLight>;

    fn as_directional_mut(&mut self)->Option<&mut DirectionalLight>;
    fn as_spot_mut(&mut self)->Option<&mut SpotLight>;
    fn as_point_mut(&mut self)->Option<&mut PointLight>;
}