use std::fmt::{Debug, Formatter, Display, Error};

pub struct Vector2{ data:[f32; 2],}
pub struct Vector3{ data:[f32; 3], }
pub struct Quantanium {data:[f32; 4],}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub trait Vector{
    fn zero()->Self;
    fn size(&self)->usize;
    fn get(&self, index:usize)->f32;
    fn set(&mut self, index:usize, value:f32);
    
    fn length(&self)->f32{
        let mut init:f32 = 0.0;
        for i in 0..self.size(){
            init += f32::powf(self.get(i), 2.0);
        }
        init
    }
    
    fn add<T:Vector>(&mut self, vector:T){
    	if self.size() == vector.size(){
    		for i in 0..self.size(){
    			self.set(i, self.get(i) + vector.get(i));
    		}
    	}else{
    		println!("Vector mismatch");
    	}
    }
    
    fn subtract<T:Vector>(&mut self, vector:T){
    	if self.size() == vector.size(){
    		for i in 0..self.size(){
    			self.set(i, self.get(i) - vector.get(i));
    		}
    	}else{
    		println!("Vector mismatch");
    	}
    }
    
    fn dot<T:Vector>(&self, vector:T) -> Option<f32>{
        if self.size() == vector.size(){
            let mut init:f32 = 0.0;
            for i in 0..self.size(){
                init += self.get(i) * vector.get(i);
            }
            return Some(init);
        }
        println!("Vector mismatch");
        None
    }
    
    fn normalize(&mut self){
        let length = self.length();
        for i in 0..self.size(){
        	self.set(i, self.get(i)/ length);
        }
    }
    
    fn crossWithVector<T:Vector>(&self, vector:T) -> Self;
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Vector2{
	pub fn new( x:f32, y:f32)->Self{
		Vector2{ data:[x, y] }
	}
	
	pub fn setX(&mut self, x:f32){ self.data[0] = x }
	pub fn setY(&mut self, y:f32){ self.data[1] = y }
	pub fn getX(&self)->f32{ self.data[0]}
	pub fn getY(&self)->f32{ self.data[1]}
	
	pub fn set(&mut self, x:f32, y:f32){
		self.data = [ x, y];
	}
	
	pub fn add(&mut self, x:f32, y:f32){
		self.data[0] += x;
		self.data[1] += y;
	}
	
	pub fn substract(&mut self, x:f32, y:f32){
		self.data[0] -= x;
		self.data[1] -= y;
	}
	
	pub fn dot(&self, x:f32, y:f32)-> f32{
		x * self.data[0] + y * self.data[1]
	}
	
	pub fn cross(&self, x:f32, y:f32)-> Vector2{
		Vector2{data: [self.data[1] - y, -(self.data[0] - x)]}
	}
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Vector for Vector2{
	fn zero()->Self{ Vector2{ data:[0.0, 0.0]}}
	
	fn size(&self)->usize{ 2 }
	
    fn get(&self, index:usize)->f32 { self.data[index]}
    fn set(&mut self, index:usize, value:f32) { self.data[index] = value;}
    
    fn crossWithVector<T:Vector>(&self, vector:T)->Self{
        Vector2::new(self.data[1] * vector.get(1), -(self.data[0] * vector.get(0)))
    }
}

impl Display for Vector2{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
		write!(f, "Vector2: {}, {}", self.getX(), self.getY())
	}
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Vector3{
    pub fn new(x:f32, y:f32, z:f32)->Self{
        Vector3{data:[x, y, z]}
    }
    
    pub fn setX(&mut self, x:f32){ self.data[0] = x }
	pub fn setY(&mut self, y:f32){ self.data[1] = y }
	pub fn setZ(&mut self, z:f32){ self.data[2] = z }
	
	pub fn getX(&self)->f32{ self.data[0]}
	pub fn getY(&self)->f32{ self.data[1]}
	pub fn getZ(&self)->f32{ self.data[2]}
	
	pub fn set(&mut self, x:f32, y:f32, z:f32){
		self.data = [ x, y, z];
	}
	
	pub fn add(&mut self, x:f32, y:f32, z:f32){
		self.data[0] += x;
		self.data[1] += y;
		self.data[2] += z;
	}
	
	pub fn substract(&mut self, x:f32, y:f32, z:f32){
		self.data[0] -= x;
		self.data[1] -= y;
		self.data[2] -= z;
	}
	
	pub fn dot(&self, x:f32, y:f32, z:f32)-> f32{
		x * self.data[0] + y * self.data[1] + z * self.data[2]
	}
	
	pub fn cross(&self, x:f32, y:f32, z:f32)-> Vector3{
		let init_x = (self.data[1] * z) - (self.data[2] * y);
        let init_y = (self.data[0] * z) - (self.data[2] * x);
        let init_z = (self.data[0] * y) - (self.data[1] * x);
		Vector3{data: [init_x, -init_y, init_z]}
	}
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Vector for Vector3{
	fn zero()->Self{ Vector3{ data:[0.0, 0.0, 0.0]}}

	fn size(&self)->usize{ 3 }

	fn get(&self, index:usize)->f32 { self.data[index]}
	fn set(&mut self, index:usize, value:f32) { self.data[index] = value;}

	fn crossWithVector<T:Vector>(&self, vector:T)->Self{
		let init_x = (self.data[1] * vector.get(2)) - (self.data[2] * vector.get(1));
		let init_y = (self.data[0] * vector.get(2)) - (self.data[2] * vector.get(0));
		let init_z = (self.data[0] * vector.get(1)) - (self.data[1] * vector.get(0));
		Vector3{data: [init_x, -init_y, init_z]}
	}
}

impl Display for Vector3{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
		write!(f, "Vector3: {}, {}, {}", self.getX(), self.getY(), self.getZ())
	}
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Quantanium{
	pub fn new(x:f32, y:f32, z:f32, w:f32)->Self{
		Quantanium{data:[x, y, z, w]}
	}

	pub fn fromVector3(vector: Vector3, w:f32)->Self{
		Quantanium{data:[vector.getX(), vector.getY(), vector.getZ(), w]}
	}

	pub fn setX(&mut self, x:f32){ self.data[0] = x; }
	pub fn setY(&mut self, y:f32){ self.data[1] = y; }
	pub fn setZ(&mut self, z:f32){ self.data[2] = z; }
	pub fn setW(&mut self, w:f32){ self.data[3] = w; }

	pub fn getX(&self)->f32{ self.data[0]}
	pub fn getY(&self)->f32{ self.data[1]}
	pub fn getZ(&self)->f32{ self.data[2]}
	pub fn getW(&self)->f32{ self.data[3]}

	pub fn set(&mut self, x:f32, y:f32, z:f32, w:f32){
		self.data = [ x, y, z, w];
	}

	pub fn add(&mut self, x:f32, y:f32, z:f32, w:f32){
		self.data[0] += x;
		self.data[1] += y;
		self.data[2] += z;
		self.data[3] += w;
	}

	pub fn substract(&mut self, x:f32, y:f32, z:f32, w:f32){
		self.data[0] -= x;
		self.data[1] -= y;
		self.data[2] -= z;
		self.data[3] -= w;
	}

	pub fn dot(&self, x:f32, y:f32, z:f32, w:f32)-> f32{
		x * self.data[0] + y * self.data[1] + z * self.data[2] + w * self.data[3]
	}

	pub fn cross(&self, x:f32, y:f32, z:f32, w:f32)-> Quantanium{
		let newW = self.data[3] * w - self.data[0] * x - self.data[1] * y - self.data[2] * z;
		let newX = self.data[3] * x + self.data[0] * w - self.data[2] * y + self.data[1] * z;
		let newY = self.data[3] * y + self.data[1] * w - self.data[0] * z + self.data[2] * x;
		let newZ = self.data[3] * z + self.data[2] * w - self.data[1] * x + self.data[0] * y;
		Quantanium{data: [newX, newY, newZ, newW]}
	}
}

#[allow(non_snake_case)]
#[allow(dead_code)]
impl Vector for Quantanium{
	fn zero() -> Self { Quantanium{ data:[0.0, 0.0, 0.0, 1.0] } }
	fn size(&self) -> usize { 4 }
	fn get(&self, index: usize) -> f32 { self.data[index] }
	fn set(&mut self, index: usize, value: f32) { self.data[index] = value; }

	fn crossWithVector<T: Vector>(&self, vector: T) -> Self {
		let newW = - self.data[0] * vector.get(0) - self.data[1] * vector.get(1) - self.data[2] * vector.get(2);
		let newX = self.data[3] * vector.get(0) - self.data[2] * vector.get(1) + self.data[1] * vector.get(2);
		let newY = self.data[3] * vector.get(1) - self.data[0] * vector.get(2) + self.data[2] * vector.get(0);
		let newZ = self.data[3] * vector.get(2) - self.data[1] * vector.get(0) + self.data[0] * vector.get(1);
		Quantanium{data: [newX, newY, newZ, newW]}
	}
}