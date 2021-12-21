use crate::grafx::physics::Color;
use crate::grafx::physics::Vector3;

#[allow(non_snake_case)]
pub enum LightType{ Directional, Point, Spot}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub trait Light{
    fn getID(&self)->&str;
    fn getType(&self)->LightType;
    fn setColor(&mut self, color:Color);
    fn getColor(&self)->&Color;
    fn setIntensity(&mut self, intensity:f32);
    fn getIntensity(&self)->f32;
}

#[allow(non_snake_case)]
pub struct DirectionalLight{id:String, color:Box<Color>, intensity:f32, direction:Box<Vector3>}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl DirectionalLight{
    pub fn new()->Self{
        let mut n = nuid::NUID::new();
        DirectionalLight{ id:n.next(), color:Box::new(Color::White()), intensity:1.0, direction:Box::new(Vector3::zero())}
    }

    pub fn setDirection(&mut self, x:f32, y:f32, z:f32){ self.direction = Box::new(Vector3::new(x, y, z)); }
}

impl Light for DirectionalLight{
    fn getID(&self)->&str{ self.id.as_ref()}
    fn getType(&self)->LightType{ LightType::Directional }
    fn setColor(&mut self, color:Color){ self.color = Box::new(color);}
    fn getColor(&self)->&Color{ &self.color }
    fn setIntensity(&mut self, intensity:f32){ self.intensity = intensity; }
    fn getIntensity(&self)->f32{ self.intensity }
}

#[allow(non_snake_case)]
pub struct PointLight{ id:String, color:Box<Color>, intensity:f32, position:Box<Vector3>, maxDistance:f32 }

#[allow(non_snake_case)]
#[allow(dead_code)]
impl PointLight{
    pub fn new()->Self{
        PointLight{ id:nuid::NUID::new().next(),  color:Box::new(Color::White()), intensity:1.0, position:Box::new(Vector3::zero()), maxDistance: 7.0}
    }

    pub fn getLinear(&self)->f32{ return 4.5 / self.maxDistance}
    pub fn getQuadratic(&self)->f32{ return (8.5 / self.maxDistance).powf(2.0); }
    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){ self.position = Box::new(Vector3::new(x, y, z));}
    pub fn getPosition(&self)->&Box<Vector3>{ &self.position}
    pub fn setMaxDistance(&mut self, distance:f32){ self.maxDistance = distance; }
}

impl Light for PointLight{
    fn getID(&self)->&str{self.id.as_ref()}
    fn getType(&self)->LightType{ LightType::Point }
    fn setColor(&mut self, color:Color){ self.color = Box::new(color);}
    fn getColor(&self)->&Color{ &self.color }
    fn setIntensity(&mut self, intensity:f32){ self.intensity = intensity; }
    fn getIntensity(&self)->f32{ self.intensity }
}

#[allow(non_snake_case)]
pub struct SpotLight{  id:String, color:Box<Color>, intensity:f32, position:Box<Vector3>, direction:Box<Vector3>, maxDistance:f32, radius:f32}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl SpotLight{
    pub fn new()->Self{
        SpotLight{
            id:nuid::NUID::new().next(),
            color:Box::new(Color::White()),
            intensity:1.0,
            position:Box::new(Vector3::zero()),
            direction:Box::new(Vector3::zero()),
            maxDistance: 0.7,
            radius:30.0
        }
    }

    pub fn getLinear(&self)->f32{ return 4.5 / self.maxDistance}

    pub fn getQuadratic(&self)->f32{
        return (8.5 / self.maxDistance).powf(2.0);
    }

    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){
        self.position = Box::new(Vector3::new(x, y, z));
    }

    pub fn setDirection(&mut self, x:f32, y:f32, z:f32){
        self.direction = Box::new(Vector3::new(x, y, z))
    }

    pub fn getPosition(&self)->&Box<Vector3>{ &self.position}
    pub fn getDirection(&self)->&Box<Vector3>{ &self.direction}
    pub fn setMaxDistance(&mut self, distance:f32){ self.maxDistance = distance; }

    pub fn getRadius(&self)->f32{ self.radius}
    pub fn getInnerRadius(&self)->f32{ f32::cos(f32::to_radians(self.radius/2.0))}
    pub fn getOuterRadius(&self)->f32{ f32::cos(f32::to_radians(self.radius))}

    pub fn setRadius(&mut self, radius:f32){ self.radius = radius; }
}

impl Light for SpotLight{
    fn getID(&self)->&str{ self.id.as_ref() }
    fn getType(&self)->LightType{ LightType::Spot }
    fn setColor(&mut self, color:Color){ self.color = Box::new(color);}
    fn getColor(&self)->&Color{ &self.color }
    fn setIntensity(&mut self, intensity:f32){ self.intensity = intensity; }
    fn getIntensity(&self)->f32{ self.intensity }
}