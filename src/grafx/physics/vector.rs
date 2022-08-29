use std::ops::{Mul, Sub, Add};

pub struct Color{ pub red:f32, pub green:f32, pub blue:f32, pub alpha:f32}

#[allow(dead_code)]
impl Color{
    pub fn new(red:f32, green:f32, blue:f32, alpha:f32)->Self{
        Color{red, green, blue, alpha}
    }

    pub fn white()->Self{
        Color{red:1.0, green:1.0, blue:1.0, alpha:1.0,}
    }
    pub fn black()->Self{
        Color{red:0.0, green:0.0, blue:0.0, alpha:1.0,}
    }
    pub fn red()->Self{
        Color{red:1.0, green:0.0, blue:0.0, alpha:1.0,}
    }
    pub fn blue()->Self{
        Color{red:0.0, green:0.0, blue:1.0, alpha:1.0,}
    }
    pub fn green()->Self{
        Color{red:1.0, green:1.0, blue:1.0, alpha:1.0,}
    }
}

#[allow(non_snake_case)]
pub struct Vector2{ data:[f32; 2]}

#[allow(non_snake_case)]
impl Vector2{
    pub fn new(x:f32, y:f32)->Self{ Vector2{ data:[x, y] } }
    pub fn zero()->Self{ Vector2{ data:[0.0, 0.0]} }
    pub fn get_x(&self)->f32{ self.data[0] }
    pub fn get_y(&self)->f32{ self.data[1] }

    pub fn set_x(&mut self, value:f32){ self.data[0] = value; }
    pub fn set_y(&mut self, value:f32){ self.data[1] = value; }
    pub fn set(&mut self, x:f32, y:f32){
        self.data[0] = x;
        self.data[1] = y;
    }

    pub fn add(&mut self, x:f32, y:f32){ self.set(x + self.get_x(), y + self.get_y()); }

    pub fn sub(&mut self, x:f32, y:f32){
        self.set(self.get_x() - x, self.get_y() - y);
    }

    pub fn length(&self)->f32{
        f32::sqrt(f32::powf(self.data[0], 2.0) + f32::powf(self.data[1], 2.0))
    }

    pub fn dot(&self, vector: Vector2)->f32{
        (vector.data[0] * self.data[0]) + (vector.data[1] * self.data[1])
    }

    pub fn normalize(&mut self){
        let length = self.length();
        self.data[0] /= length;
        self.data[1] /= length;
    }
}

impl Mul for &Vector2{
    type Output = Vector2;
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.get_y() - rhs.get_y();
        let y = self.get_x() - rhs.get_x();
        return Vector2{ data:[x, -y]};
    }
}

impl Mul<f32> for &Vector2{
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        return Vector2{ data:[self.data[0] * rhs, self.data[1] * rhs]};
    }
}

impl Add for &Vector2{
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        return Vector2{ data:[self.data[0] + rhs.data[0], self.data[1] + rhs.data[1]]};
    }
}

impl Sub for &Vector2{
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        return Vector2{ data:[self.data[0] - rhs.data[0], self.data[1] - rhs.data[1]]};
    }
}

#[allow(non_snake_case)]
pub struct Vector3{ data:[f32; 3]}

#[allow(non_snake_case)]
impl Vector3{
    pub fn new(x:f32, y:f32, z:f32)->Self{ Vector3{data:[x, y, z]} }
    pub fn zero()->Self{ Vector3{ data:[0.0, 0.0, 0.0]} }
    pub fn up()->Self{ Vector3{ data:[0.0, 1.0, 0.0]} }
    pub fn one()->Self{ Vector3{ data:[1.0, 1.0, 1.0]} }

    pub fn get_x(&self)->f32{ self.data[0] }
    pub fn get_y(&self)->f32{ self.data[1] }
    pub fn get_z(&self)->f32{ self.data[2] }

    pub fn set_x(&mut self, value:f32){ self.data[0] = value; }
    pub fn set_y(&mut self, value:f32){ self.data[1] = value; }
    pub fn set_z(&mut self, value:f32){ self.data[2] = value;}
    pub fn set(&mut self, x:f32, y:f32, z:f32){
        self.data[0] = x;
        self.data[1] = y;
        self.data[2] = z;
    }

    pub fn add(&mut self, x:f32, y:f32, z:f32){
        self.set(x + self.get_x(), y + self.get_y(), z + self.get_z());
    }

    pub fn sub(&mut self, x:f32, y:f32, z:f32){
        self.set(self.get_x() - x, self.get_y() - y, self.get_z() - z);
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
        let x = (self.get_y() * rhs.get_z()) - (self.get_z() * rhs.get_y());
        let y = (self.get_x() * rhs.get_z()) - (self.get_z() * rhs.get_x());
        let z = (self.get_x() * rhs.get_y()) - (self.get_y() * rhs.get_x());
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