
use crate::grafx::physics::{ Color, Vector3};

pub struct Directional{ direction:Box<Vector3> }
#[allow(non_snake_case)]
impl Directional{
    pub fn setDirection(&mut self, x:f32, y:f32, z:f32){ self.direction = Box::new(Vector3::new(x, y, z)); }
    pub fn getDirection(&self)->&Box<Vector3>{ &self.direction}
}

#[allow(non_snake_case)]
pub struct Point{ position:Box<Vector3>, maxDistance:f32}
#[allow(non_snake_case)]
impl Point{
    pub fn getLinear(&self)->f32{ return 4.5 / self.maxDistance}
    pub fn getQuadratic(&self)->f32{ return (8.5 / self.maxDistance).powf(2.0); }
    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){ self.position = Box::new(Vector3::new(x, y, z));}
    pub fn getPosition(&self)->&Box<Vector3>{ &self.position}
    pub fn setMaxDistance(&mut self, distance:f32){ self.maxDistance = distance; }
}

#[allow(non_snake_case)]
pub struct Spot{  position:Box<Vector3>, direction:Box<Vector3>, maxDistance:f32, radius:f32 }
#[allow(non_snake_case)]
#[allow(dead_code)]
impl Spot{
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

#[allow(non_snake_case)]
pub enum LightType{ DirectionalLight(Directional), PointLight(Point), SpotLight(Spot), }

#[allow(non_snake_case)]
#[allow(dead_code)]
impl LightType{
    pub fn newDirectional()->Self{
        LightType::DirectionalLight(Directional{ direction:Box::new(Vector3::zero())})
    }

    pub fn newPoint()->Self{
        LightType::PointLight(Point{position:Box::new(Vector3::zero()), maxDistance: 7.0 })
    }

    pub fn newSpot()->Self{
        LightType::SpotLight(Spot{ position:Box::new(Vector3::zero()), direction:Box::new(Vector3::zero()), maxDistance: 0.7, radius:30.0 })
    }
}

#[allow(non_snake_case)]
pub struct Light{ id:String, lightType:LightType, color:Box<Color>, intensity:f32, ambientStrenght:f32}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Light{
    pub fn new(lightType:LightType)->Self{
        let mut n = nuid::NUID::new();
        Light{ id:n.next(), lightType, color:Box::new(Color::White()), intensity:1.0, ambientStrenght: 0.2}
    }

    pub fn getID(&self)->&str{ self.id.as_ref()}
    pub fn getType(&self)->&LightType{ &self.lightType }
    pub fn setColor(&mut self, color:Color){ self.color = Box::new(color);}
    pub fn getColor(&self)->&Color{ &self.color }
    pub fn setIntensity(&mut self, intensity:f32){ self.intensity = intensity; }
    pub fn getIntensity(&self)->f32{ self.intensity }
    pub fn getAmbientStrenght(&self)->f32{ self.ambientStrenght}
    pub fn setAmbientStrenght(&mut self, ambientStrenght:f32){ self.ambientStrenght = ambientStrenght}

    pub fn setDirection(&mut self, x:f32, y:f32, z:f32){
        if let LightType::DirectionalLight(direction) = &mut self.lightType {
            direction.setDirection(x, y, z);
        }else if let LightType::SpotLight(spot)= &mut self.lightType {
            spot.setDirection(x, y, z);
        }
    }

    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){
        if let LightType::PointLight(point) = &mut self.lightType {
            point.setPosition(x, y, z);
        }else if let LightType::SpotLight(spot)= &mut self.lightType {
            spot.setPosition(x,y,z);
        }
    }

    pub fn setMaxDistance(&mut self, distance:f32){
        if let LightType::PointLight(point) = &mut self.lightType {
            point.setMaxDistance(distance);
        }else if let LightType::SpotLight(spot)= &mut self.lightType {
            spot.setMaxDistance(distance);
        }
    }

    pub fn setRadius(&mut self, radius:f32){
        if let LightType::SpotLight(spot)= &mut self.lightType {
            spot.setRadius(radius)
        }
    }
}