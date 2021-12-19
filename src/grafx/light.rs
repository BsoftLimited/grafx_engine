#[allow(non_snake_case)]
pub enum LightType{ Directional, Point, Spot}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub trait Light{
    fn getID(&self)->&str;
    fn getType(&self)->LightType;
}

#[allow(non_snake_case)]
pub struct DirectionalLight{ id:String,}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl DirectionalLight{
    pub fn new()->Self{
        let mut n = nuid::NUID::new();
        DirectionalLight{ id:n.next()}
    }
}

impl Light for DirectionalLight{
    fn getID(&self)->&str{
        self.id.as_ref()
    }

    fn getType(&self)->LightType{ LightType::Directional }
}

#[allow(non_snake_case)]
pub struct SpotLight{ id:String,}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl SpotLight{
    pub fn new()->Self{
        let mut n = nuid::NUID::new();
        SpotLight{ id:n.next()}
    }
}

impl Light for SpotLight{
    fn getID(&self)->&str{
        self.id.as_ref()
    }

    fn getType(&self)->LightType{ LightType::Spot }
}

#[allow(non_snake_case)]
pub struct PointLight{ id:String,}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl PointLight{
    pub fn new()->Self{
        let mut n = nuid::NUID::new();
        PointLight{ id:n.next()}
    }
}

impl Light for PointLight{
    fn getID(&self)->&str{
        self.id.as_ref()
    }

    fn getType(&self)->LightType{ LightType::Point }
}