use std::ops::{Mul, Sub, Add};

#[allow(non_snake_case)]
pub struct Color{ pub red:f32, pub green:f32, pub blue:f32, pub alpha:f32}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Color{
    pub fn new(red:f32, green:f32, blue:f32, alpha:f32)->Self{
        Color{red, green, blue, alpha}
    }

    pub fn White()->Self{
        Color{red:1.0, green:1.0, blue:1.0, alpha:1.0,}
    }
    pub fn Black()->Self{
        Color{red:0.0, green:0.0, blue:0.0, alpha:1.0,}
    }
    pub fn Red()->Self{
        Color{red:1.0, green:0.0, blue:0.0, alpha:1.0,}
    }
    pub fn Blue()->Self{
        Color{red:0.0, green:0.0, blue:1.0, alpha:1.0,}
    }
    pub fn Green()->Self{
        Color{red:1.0, green:1.0, blue:1.0, alpha:1.0,}
    }
}

#[allow(non_snake_case)]
pub struct Vector3{ data:[f32; 3]}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Vector3{
    pub fn new(x:f32, y:f32, z:f32)->Self{
        Vector3{data:[x, y, z]}
    }
    pub fn zero()->Self{
        Vector3{ data:[0.0, 0.0, 0.0]}
    }
    pub fn up()->Self{
        Vector3{ data:[0.0, 1.0, 0.0]}
    }
    pub fn one()->Self{
        Vector3{ data:[1.0, 1.0, 1.0]}
    }

    pub fn getX(&self)->f32{
        self.data[0]
    }
    pub fn getY(&self)->f32{
        self.data[1]
    }
    pub fn getZ(&self)->f32{
        self.data[2]
    }

    pub fn setX(&mut self, value:f32){
        self.data[0] = value;
    }
    pub fn setY(&mut self, value:f32){
        self.data[1] = value;
    }
    pub fn setZ(&mut self, value:f32){
        self.data[2] = value;
    }
    pub fn set(&mut self, x:f32, y:f32, z:f32){
        self.data[0] = x;
        self.data[1] = y;
        self.data[2] = z;
    }

    pub fn addV(&mut self, x:f32, y:f32, z:f32){
        self.set(x + self.getX(), y + self.getY(), z + self.getZ());
    }

    pub fn sub(&mut self, x:f32, y:f32, z:f32){
        self.set(self.getX() - x, self.getY() - y, self.getZ() - z);
    }

    pub fn length(&self)->f32{
        f32::sqrt(f32::powf(self.data[0], 2.0) + f32::powf(self.data[1], 2.0) + f32::powf(self.data[2], 2.0))
    }

    pub fn dot(&self, vector: Vector3)->f32{
        (vector.data[0] * self.data[0]) + (vector.data[1] * self.data[1]) + (vector.data[2] * self.data[2])
    }

    pub fn normalize(&mut self){
        let length = self.length();
        self.data[0] /= length;
        self.data[1] /= length;
        self.data[2] /= length;
    }
}

impl Mul for &Vector3{
    type Output = Vector3;
    fn mul(self, rhs: Self) -> Self::Output {
        let x = (self.getY() * rhs.getZ()) - (self.getZ() * rhs.getY());
        let y = (self.getX() * rhs.getZ()) - (self.getZ() * rhs.getX());
        let z = (self.getX() * rhs.getY()) - (self.getY() * rhs.getX());
        return Vector3{ data:[x, -y, z]};
    }
}

impl Mul<f32> for &Vector3{
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Self::Output {
        return Vector3{ data:[self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs]};
    }
}

impl Add for &Vector3{
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector3{ data:[self.data[0] + rhs.data[0], self.data[1] + rhs.data[1], self.data[2] + rhs.data[2]]};
    }
}

impl Sub for &Vector3{
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector3{ data:[self.data[0] - rhs.data[0], self.data[1] - rhs.data[1], self.data[2] - rhs.data[2]]};
    }
}