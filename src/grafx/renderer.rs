use crate::grafx::OrthographicCamera;
use crate::PerspectiveCamera;
use crate::grafx::light::SpotLight;
use crate::grafx::light::PointLight;
use crate::grafx::light::DirectionalLight;
use std::collections::HashMap;
use crate::grafx::light::Light;
use crate::grafx::{Camera};
use std::collections::hash_map::Iter;

pub trait Renderer<T: Camera>{
    fn set_camera(&mut self, camera:T);
    fn get_camera(&mut self)->&mut T;
    fn get_view(&self)->&[[f32; 4]; 4];
    fn get_projection(&self)->&[[f32; 4]; 4];
    fn get_lights(&self)->Iter<String, Box<dyn Light>>;

    fn add_directional_light(&mut self, light:DirectionalLight)->String;
    fn add_point_light(&mut self, light:PointLight)->String;
    fn add_spot_light(&mut self, light:SpotLight)->String;
    fn remove_light(&mut self, id:&str)->bool;
    fn get_light(&self, id: &str)->Option<&dyn Light>;
    fn get_light_mut(&mut self, id: &str)->Option<&mut dyn Light>;
}

pub struct ModelRenderer{ camera:Box<PerspectiveCamera>, lights:HashMap<String, Box<dyn Light>>}
impl ModelRenderer{
    pub fn new(camera:PerspectiveCamera)->Self{
        ModelRenderer{camera: Box::new(camera), lights:HashMap::new()}
    }
}


impl Renderer<PerspectiveCamera> for ModelRenderer{
    fn set_camera(&mut self, camera:PerspectiveCamera){ self.camera = Box::new(camera);}
    fn get_camera(&mut self)->&mut PerspectiveCamera{ self.camera.as_mut() }
    fn get_view(&self)->&[[f32; 4]; 4]{ self.camera.get_data().get_view() }
    fn get_projection(&self)->&[[f32; 4]; 4]{ self.camera.get_data().get_projection() }
    fn get_lights(&self)->Iter<String, Box<dyn Light>>{ self.lights.iter() }

    fn add_directional_light(&mut self, light:DirectionalLight)->String{
        let id = String::from(light.get_id());
        self.lights.insert(id.clone(), Box::new(light));
        return id;
    }

    fn add_point_light(&mut self, light:PointLight)->String{
        let id = String::from(light.get_id());
        self.lights.insert(id.clone(), Box::new(light));
        return id;
    }

    fn add_spot_light(&mut self, light:SpotLight)->String{
        let id = String::from(light.get_id());
        self.lights.insert(id.clone(), Box::new(light));
        return id;
    }

    fn remove_light(&mut self, id:&str)->bool{
        self.lights.remove(id);
        return false;
    }

    fn get_light(&self, id: &str)->Option<&dyn Light>{
        if self.lights.contains_key(id){
            return Some(self.lights.get(id).unwrap().as_ref());
        }
        return None;
    }

    fn get_light_mut(&mut self, id: &str)->Option<&mut dyn Light>{
        if self.lights.contains_key(id){
            return Some(self.lights.get_mut(id).unwrap().as_mut());
        }
        return None;
    }
}

pub struct SpriteRenderer{ camera:Box<OrthographicCamera>, lights:HashMap<String, Box<dyn Light>>}

#[allow(dead_code)]
impl SpriteRenderer{
    pub fn new(camera: OrthographicCamera)->Self{
        SpriteRenderer{camera: Box::new(camera), lights:HashMap::new()}
    }
}

impl Renderer<OrthographicCamera> for SpriteRenderer{
    fn set_camera(&mut self, camera: OrthographicCamera){ self.camera = Box::new(camera);}
    fn get_camera(&mut self)->&mut OrthographicCamera{ self.camera.as_mut() }
    fn get_view(&self)->&[[f32; 4]; 4]{ self.camera.get_data().get_view() }
    fn get_projection(&self)->&[[f32; 4]; 4]{ self.camera.get_data().get_projection() }
    fn get_lights(&self)->Iter<String, Box<dyn Light>>{ self.lights.iter() }

    fn add_directional_light(&mut self, light:DirectionalLight)->String{
        let id = String::from(light.get_id());
        self.lights.insert(id.clone(), Box::new(light));
        return id;
    }

    fn add_point_light(&mut self, light:PointLight)->String{
        let id = String::from(light.get_id());
        self.lights.insert(id.clone(), Box::new(light));
        return id;
    }

    fn add_spot_light(&mut self, light:SpotLight)->String{
        let id = String::from(light.get_id());
        self.lights.insert(id.clone(), Box::new(light));
        return id;
    }

    fn remove_light(&mut self, id:&str)->bool{
        self.lights.remove(id);
        return false;
    }

    fn get_light(&self, id: &str)->Option<&dyn Light>{
        if self.lights.contains_key(id){
            return Some(self.lights.get(id).unwrap().as_ref());
        }
        return None;
    }

    fn get_light_mut(&mut self, id: &str)->Option<&mut dyn Light>{
        if self.lights.contains_key(id){
            return Some(self.lights.get_mut(id).unwrap().as_mut());
        }
        return None;
    }
}