extern crate gl;
extern crate glutin;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ ControlFlow, EventLoop};
use glutin::window::{WindowBuilder};
use glutin::{ContextBuilder, WindowedContext, PossiblyCurrent, ContextWrapper};

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::os::raw::c_void;
use std::ffi::CString;
use std::ops::{Mul, Add, Sub};use std::fmt::{Display, Formatter, Error};
use std::time::{SystemTime, UNIX_EPOCH};

static mut DELTA_TIME: f64 = 0.0;
static mut LAST_TIME:f64 = 0.0;

#[allow(non_snake_case)]
struct Color{ pub red:f32, pub green:f32, pub blue:f32, pub alpha:f32}
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
struct Vector3{ data:[f32; 3]}
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

#[allow(non_snake_case)]
trait Matrix{
    fn identity()->Self;
    fn zero()->Self;
    fn setValue(&mut self, row:usize, col:usize, value:f32);
    fn getValue(&self, row:usize, col:usize)->f32;
    fn determinant(&self)->f32;
    fn transpose(&self)->Self;
    fn coefficient(&self)->Self;
    fn inverse(&self)->Self;
}

#[allow(non_snake_case)]
struct Matrix2{ rowCount: usize, columnCount: usize, data:[[f32; 2]; 2], }
#[allow(non_snake_case)]
impl Matrix for Matrix2{
    fn identity()->Self{ Matrix2{ rowCount: 2, columnCount:2, data:[ [1.0, 0.0],      [0.0, 1.0]]} }
    fn zero()->Self{
        Matrix2{ rowCount: 2, columnCount:2, data:[ [0.0, 0.0],      [0.0, 0.0]]}
    }

    fn setValue(&mut self, row: usize, col: usize, value:f32){
        self.data[row][col] = value;
    }
    fn getValue(&self, row: usize, col: usize,)->f32{
        self.data[row][col]
    }

    fn determinant(&self)->f32{
        (self.getValue(0, 0) * self.getValue(1, 1)) - (self.getValue(0, 1) * self.getValue(1, 0))
    }

    fn transpose(&self) -> Self {
        let mut matrix = Matrix2::zero();
        for i in 0..2{
            for j in 0..2 {
                if i != j {
                    matrix.setValue(j, i, self.getValue(i, j));
                }else {
                    matrix.setValue(i, j, self.getValue(i, j));
                }
            }
        }
        matrix
    }

    fn coefficient(&self) -> Self {
        let mut matrix = Matrix2::zero();
        matrix.setValue(0, 0, self.getValue(1, 1));
        matrix.setValue(0, 1, -self.getValue(1, 0));
        matrix.setValue(1, 0, -self.getValue(0, 1));
        matrix.setValue(1, 1, self.getValue(0, 0));
        matrix
    }

    fn inverse(&self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }
}

impl Mul for &Matrix2{
    type Output = Matrix2;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut init = Matrix2::zero();
        for i in 0..2 {
            for j in 0..2{
                let mut value = 0.0;
                for k in 0..2{
                    value += self.getValue(i, k) * rhs.getValue(k, j);
                }
                init.setValue(i, j, value);
            }
        }
        return init;
    }
}

impl Mul<f32> for &Matrix2{
    type Output = Matrix2;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut init = Matrix2::zero();
        for i in 0..2{
            for j in 0..2{
                init.setValue(i, j, self.getValue(i, j) * rhs);
            }
        }
        return init;
    }
}

impl Display for Matrix2{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}\n{}, {}", self.getValue(0, 0), self.getValue(0, 1), self.getValue(1,0), self.getValue(1,1))
    }
}

#[allow(non_snake_case)]
struct Matrix3{ rowCount: usize, columnCount: usize, data:[[f32; 3]; 3], }
#[allow(non_snake_case)]
impl Matrix for Matrix3{
    fn identity()->Self{
        Matrix3{ rowCount: 3, columnCount:3, data:[ [1.0, 0.0, 0.0],      [0.0, 1.0, 0.0],     [0.0, 0.0, 1.0]]}
    }

    fn zero()->Self{
        Matrix3{ rowCount: 3, columnCount:3, data:[ [0.0, 0.0, 0.0],      [0.0, 0.0, 0.0],     [0.0, 0.0, 0.0]]}
    }

    fn setValue(&mut self, row: usize, col: usize, value:f32){
        self.data[row][col] = value;
    }

    fn getValue(&self, row: usize, col: usize,)->f32{
        self.data[row][col]
    }

    fn determinant(&self)->f32{
        let (mut sign, mut value) = (-1.0, 0.0);
        for i in 0..3 {
            let mut init = Matrix2::zero();
            sign *= -1.0;
            for k in 0..3 {
                for l in 0..3{
                    if l != i {
                        let mut b= 0;
                        if l < i { b = l; }else if l > i { b = l - 1; }
                        init.setValue(k-1,b, self.getValue(k, l));
                    }
                }
            }
            value += self.getValue(0, i) * sign * init.determinant();
        }
        return value;
    }

    fn transpose(&self) -> Self {
        let mut matrix = Matrix3::zero();
        for i in 0..3{
            for j in 0..3 {
                if i != j {
                    matrix.setValue(j, i, self.getValue(i, j));
                }else {
                    matrix.setValue(i, j, self.getValue(i, j));
                }
            }
        }
        matrix
    }

    fn coefficient(&self) -> Self {
        let (mut sign, mut matrix) = (1.0, Matrix3::zero());
        for i in 0..self.rowCount{
            let mut sub = sign;
            sign *= -1.0;
            for j in 0..self.rowCount {
                let mut init = Matrix2::zero();
                for k in 0..self.columnCount {
                    for l in 0..self.columnCount{
                        if k != i && l != j {
                            let (mut a , mut b) = (0, 0);
                            if k < i {
                                a = k;
                            } else if k > i {
                                a = k - 1;
                            }
                            if l < j {
                                b = l;
                            } else if l > j {
                                b = l - 1;
                            }
                            init.setValue(a, b, self.getValue(k, l));
                        }
                    }
                }
                matrix.setValue(i, j, sub * init.determinant());
                sub *= -1.0;
            }
        }
        matrix
    }

    fn inverse(&self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }
}

impl Mul for &Matrix3{
    type Output = Matrix3;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut init = Matrix3::zero();
        for i in 0..3 {
            for j in 0..3{
                let mut value = 0.0;
                for k in 0..3{
                    value += self.getValue(i, k) * rhs.getValue(k, j);
                }
                init.setValue(i, j, value);
            }
        }
        init
    }
}

impl Mul<f32> for &Matrix3{
    type Output = Matrix3;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut init = Matrix3::zero();
        for i in 0..4{
            for j in 0..4{
                init.setValue(i, j, self.getValue(i, j) * rhs);
            }
        }
        return init;
    }
}

impl Display for Matrix3{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}, {}\n{}, {}, {}\n{}, {}, {}",
               self.getValue(0, 0), self.getValue(0, 1), self.getValue(0, 2),
               self.getValue(1,0), self.getValue(1,1), self.getValue(1,2),
               self.getValue(2,0), self.getValue(2,1), self.getValue(2,2))
    }
}

#[allow(non_snake_case)]
struct Matrix4{ rowCount: usize, columnCount: usize, data:[[f32; 4]; 4], }

#[allow(non_snake_case)]
impl Matrix4{
    pub fn set(&mut self, matrix:&Matrix4){ self.data = matrix.data; }
    pub fn xRotationMatrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_radians(degree);
        matrix.setValue(1, 1, f32::cos(radian));
        matrix.setValue(1, 2, -f32::sin(radian));
        matrix.setValue(2, 1, f32::sin(radian));
        matrix.setValue(2, 2, f32::cos(radian));
        matrix
    }

    pub fn yRotationMatrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_radians(degree);
        matrix.setValue(0, 0, f32::cos(radian));
        matrix.setValue(0, 2, -f32::sin(radian));
        matrix.setValue(2, 0, f32::sin(radian));
        matrix.setValue(2, 2, f32::cos(radian));
        matrix
    }

    pub fn zRotationMatrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_radians(degree);
        matrix.setValue(0, 0, f32::cos(radian));
        matrix.setValue(0, 1, -f32::sin(radian));
        matrix.setValue(1, 0, f32::sin(radian));
        matrix.setValue(1, 1, f32::cos(radian));
        matrix
    }

    pub fn RotationMatrix(vector:&Vector3)->Self{
        Matrix4::xRotationMatrix(vector.getX()) * Matrix4::yRotationMatrix(vector.getY()) * Matrix4::zRotationMatrix(vector.getZ())
    }

    pub fn TranslateMatrix(vector: &Vector3)->Self{
        let mut matrix = Matrix4::identity();
        matrix.setValue(0, 3, vector.getX());
        matrix.setValue(1, 3, vector.getY());
        matrix.setValue(2, 3, vector.getZ());
        matrix
    }

    pub fn  ScaleMatrix(vector: &Vector3)->Self{
        let mut matrix = Matrix4::identity();
        matrix.setValue(0, 0, vector.getX());
        matrix.setValue(1, 1, vector.getY());
        matrix.setValue(2, 2, vector.getZ());
        matrix
    }

    pub fn ProjectionMatrix(fov:f32, width:f32, hieght:f32, near:f32, far:f32)->Self{
        let mut matrix = Matrix4::identity();
        let ar = width / hieght;
        let angle = 1.0 / f32::tan(f32::to_radians(fov / 2.0));
        let x = far / (far - near);
        matrix.setValue(0, 0, angle / ar);
        matrix.setValue(1, 1, angle);
        matrix.setValue(2, 2, -x);
        matrix.setValue(2, 3, -x * near);
        matrix.setValue(3, 2, -1.0);
        matrix.setValue(3, 3, 0.0);
        return matrix;
    }

    pub fn LookAtMatrix(position: &Vector3, target: &Vector3, up: &Vector3)->Self {
        let mut cameraDirection = position - target;
        cameraDirection.normalize();
        let mut cameraRight = up * &cameraDirection;
        cameraRight.normalize();
        let mut cameraUp = &cameraDirection *  &cameraRight;

        let mut matrixA = Matrix4::identity();
        matrixA.setValue(0, 0, cameraRight.getX());		matrixA.setValue(0, 1, cameraRight.getY()); 	matrixA.setValue(0, 2, cameraRight.getZ());
        matrixA.setValue(1, 0, cameraUp.getX()); 		matrixA.setValue(1, 1, cameraUp.getY()); 		matrixA.setValue(1, 2, cameraUp.getZ());
        matrixA.setValue(2, 0, cameraDirection.getX()); matrixA.setValue(2, 1, cameraDirection.getY()); matrixA.setValue(2, 2, cameraDirection.getZ());

        let mut matrixB = Matrix4::identity();
        matrixB.setValue(0, 3, -position.getX()); 		matrixB.setValue(1, 3, -position.getY());		 matrixB.setValue(2, 3, -position.getZ());
        matrixA * matrixB
    }
}

#[allow(non_snake_case)]
impl Matrix for Matrix4{
    fn identity()->Self{
        Matrix4{ rowCount: 4, columnCount:4, data:[ [1.0, 0.0, 0.0, 0.0],      [0.0, 1.0, 0.0, 0.0],     [0.0, 0.0, 1.0, 0.0],    [0.0, 0.0, 0.0, 1.0]]}
    }

    fn zero()->Self{
        Matrix4{ rowCount: 4, columnCount:4, data:[ [0.0, 0.0, 0.0, 0.0],      [0.0, 0.0, 0.0, 0.0],     [0.0, 0.0, 0.0, 0.0],    [0.0, 0.0, 0.0, 0.0]]}
    }

    fn setValue(&mut self, row: usize, col: usize, value:f32){
        self.data[row][col] = value;
    }

    fn getValue(&self, row: usize, col: usize,)->f32{
        self.data[row][col]
    }

    fn determinant(&self)->f32{
        let (mut sign, mut value) = (-1.0, 0.0);
        for i in 0..4 {
            let mut init = Matrix3::zero();
            sign *= -1.0;
            for k in 0..4 {
                for l in 0..4{
                    if l != i {
                        let mut b= 0;
                        if l < i { b = l; }else if l > i { b = l - 1; }
                        init.setValue(k-1,b, self.getValue(k, l));
                    }
                }
            }
            value += self.getValue(0, i) * sign * init.determinant();
        }
        return value;
    }

    fn transpose(&self) -> Self {
        let mut matrix = Matrix4::zero();
        for i in 0..4{
            for j in 0..4 {
                if i != j {
                    matrix.setValue(j, i, self.getValue(i, j));
                }else {
                    matrix.setValue(i, j, self.getValue(i, j));
                }
            }
        }
        matrix
    }

    fn coefficient(&self) -> Self {
        let (mut sign, mut matrix) = (1.0, Matrix4::zero());
        for i in 0..self.rowCount{
            let mut sub = sign;
            sign *= -1.0;
            for j in 0..self.rowCount {
                let mut init = Matrix3::zero();
                for k in 0..self.columnCount {
                    for l in 0..self.columnCount{
                        if k != i && l != j {
                            let (mut a , mut b) = (0, 0);
                            if k < i {
                                a = k;
                            } else if k > i {
                                a = k - 1;
                            }
                            if l < j {
                                b = l;
                            } else if l > j {
                                b = l - 1;
                            }
                            init.setValue(a, b, self.getValue(k, l));
                        }
                    }
                }
                matrix.setValue(i, j, sub * init.determinant());
                sub *= -1.0;
            }
        }
        matrix
    }

    fn inverse(&self) -> Self {
        self.coefficient().transpose() * (1.0 / self.determinant())
    }
}

impl Mul for Matrix4{
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut init = Matrix4::zero();
        for i in 0..4 {
            for j in 0..4{
                let mut value = 0.0;
                for k in 0..4{
                    value += self.getValue(i, k) * rhs.getValue(k, j);
                }
                init.setValue(i, j, value);
            }
        }
        return init;
    }
}

impl Mul<f32> for Matrix4{
    type Output = Matrix4;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut init = Matrix4::zero();
        for i in 0..4{
            for j in 0..4{
                init.setValue(i, j, self.getValue(i, j) * rhs);
            }
        }
        init
    }
}

impl Display for Matrix4{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}",
               self.getValue(0, 0), self.getValue(0, 1), self.getValue(0, 2), self.getValue(0, 3),
               self.getValue(1,0), self.getValue(1,1), self.getValue(1,2), self.getValue(1, 3),
               self.getValue(2,0), self.getValue(2,1), self.getValue(2,2), self.getValue(2, 3),
               self.getValue(3,0), self.getValue(3,1), self.getValue(3,2), self.getValue(3, 3),)
    }
}

#[allow(non_snake_case)]
struct Transformation{ transform:Box<Matrix4>, position:Box<Vector3>, rotation:Box<Vector3>, scale:Box<Vector3>}
#[allow(non_snake_case)]
impl Transformation{
    pub fn new()->Self{
        Transformation{
            transform:Box::new(Matrix4::identity()),
            position:Box::new(Vector3::zero()),
            rotation:Box::new(Vector3::zero()),
            scale:Box::new(Vector3::one()),
        }
    }

    pub fn update(&mut self){
        let mut mat = Matrix4::TranslateMatrix(&self.position) * Matrix4::RotationMatrix(&self.rotation) * Matrix4::ScaleMatrix(&self.scale);
        self.transform.data = mat.data;
    }

    pub fn getTransform(&self)->&Matrix4{ &self.transform }
    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){ self.position.set(x, y, z); }
    pub fn translate(&mut self, x:f32, y:f32, z:f32){ self.position.addV(x, y, z); }
    pub fn setRotation(&mut self, x:f32, y:f32, z:f32){ self.rotation.set(x, y, z); }
    pub fn rotate(&mut self, x:f32, y:f32, z:f32){ self.rotation.addV(x, y, z); }
    pub fn setScale(&mut self, x:f32, y:f32, z:f32){ self.scale.set(x, y, z); }
    pub fn scale(&mut self, x:f32, y:f32, z:f32){ self.scale.set(self.scale.getX() * x, self.scale.getY() * y, self.scale.getZ() * z); }
}

#[allow(non_snake_case)]
struct Camera{ view:Matrix4, projection:Matrix4, position:Vector3, target:Vector3, fov:f32, viewPointWidth:f32, viewPointHeight:f32 }
#[allow(non_snake_case)]
impl Camera{
    pub fn new(fov:f32, width:f32, height:f32)->Self{
        let projection = Matrix4::ProjectionMatrix(fov, width, height, 1.0, 1000.0);

        let position = Vector3::zero();
        let target = Vector3::zero();
        let view = Matrix4::LookAtMatrix(&position, &target, &Vector3::up());
        Camera{view, projection, position, target, fov, viewPointWidth:width, viewPointHeight:height}
    }

    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){
        self.position.set(x, y, z);
        self.view = Matrix4::LookAtMatrix(&self.position, &self.target, &Vector3::up());
    }

    pub fn translate(&mut self, x:f32, y:f32, z:f32){
        self.position.addV(x, y, z);
        self.view = Matrix4::LookAtMatrix(&self.position, &self.target, &Vector3::up());
    }

    pub fn getViewMatrix(&self)->&Matrix4{ &self.view }
    pub fn getProjectionMatrix(&self)->&Matrix4{ &self.projection }
}

#[allow(non_snake_case)]
struct Shader{ shader_program: u32}
#[allow(non_snake_case)]
impl Shader{
    pub unsafe fn new(vertex: &str, fragment: &str) -> Self{
        // Setup shader compilation checks
        let vertex_shader = Shader::compile(gl::VERTEX_SHADER, vertex);
        let fragment_shader = Shader::compile(gl::FRAGMENT_SHADER, fragment);

        let shader_program = Shader::link(vertex_shader, fragment_shader);
        Shader { shader_program }
    }

    unsafe fn compile(shaderType: u32, shaderSource:&str) -> u32{
        // Setup shader compilation checks
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // -1 to skip trialing null character

        // Vertex shader
        let shader = gl::CreateShader(shaderType);
        let c_str_vert = CString::new(shaderSource.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Check for shader compilation errors
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar, );
            println!("ERROR::SHADER::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }
        shader
    }

    unsafe fn link(vertex_shader:u32, fragment_shader:u32,) ->u32{
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // -1 to skip trialing null character

        // Link Shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar, );
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }

    pub unsafe fn setUniformValue(&self, name:&str, value: f32){
        let c_name = CString::new(name).unwrap();
        let ptr = c_name.as_ptr();
        let uniform = gl::GetUniformLocation(self.shader_program, ptr);
        gl::ProgramUniform1f(self.shader_program, uniform, value);
    }

    pub unsafe fn setUniformMatrix(&self, name:&str, matrix: &Matrix4){
        let c_name = CString::new(name).unwrap();

       let uniform = gl::GetUniformLocation(self.shader_program, c_name.as_ptr());
        gl::ProgramUniformMatrix4fv(self.shader_program, uniform, 1, gl::TRUE, std::mem::transmute(&matrix.data));
    }

    pub unsafe  fn bind(&self){
        gl::UseProgram(self.shader_program);
    }
}

struct Shape{ vao: u32, size: i32,}
impl Shape{
    pub fn new(width:f32, height:f32, depth:f32, color:Color )->Self{
        let vertices = [
            //TOP
            -width, height, depth,	0.0, 1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 1.0,
            width, height, depth,	0.0, 1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 1.0,
            width, height, -depth,	0.0, 1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 0.0,
            -width, height, -depth,	0.0, 1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 0.0,

            //BOTTOM
            -width, -height, depth,	0.0, -1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 1.0,
            width, -height, depth,	0.0, -1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 1.0,
            width, -height, -depth,	0.0, -1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 0.0,
            -width, -height, -depth,0.0, -1.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 0.0,

            //LEFT
            -width, -height, depth,	-1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 1.0,
            -width,	height, depth,	-1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 0.0,
            -width, height, -depth,	-1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 0.0,
            -width, -height, -depth,-1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 1.0,

            //RIGHT
            width, -height, depth,	1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 1.0,
            width, height, depth,	1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 0.0,
            width, height, -depth,	1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 0.0,
            width, -height, -depth,	1.0, 0.0, 0.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 1.0,

            //FRONT
            -width, height, -depth,	0.0, 0.0, -1.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 0.0,
            width, height, -depth,	0.0, 0.0, -1.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 0.0,
            width, -height, -depth,	0.0, 0.0, -1.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 1.0,
            -width, -height, -depth,0.0, 0.0, -1.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 1.0,

            //BACK
            -width, height, depth,	0.0, 0.0, 1.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 0.0,
            width, height, depth, 	0.0, 0.0, 1.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 0.0,
            width, -height, depth,	0.0, 0.0, 1.0,	color.red, color.green, color.blue, color.alpha,	 1.0, 1.0,
            -width, -height, depth,	0.0, 0.0, 1.0,	color.red, color.green, color.blue, color.alpha,	 0.0, 1.0
        ];

        let indices:[u32; 36] = [
            2, 1, 0,		0, 3, 2,		//Top
            4, 5, 6,		6, 7, 4,		//Buttom
            10, 9, 8,		8, 11, 10,		//Left
            12, 13, 14,		14, 15, 12,		//Right
            18, 17, 16,		16, 19, 18,		//Front
            20, 21, 22,		22, 23, 20		//Back
        ];

        let (mut vbo, mut ebo, mut vao) = (0, 0, 0);
        unsafe{
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW, );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<GLint>()) as GLsizeiptr, &indices[0] as *const u32 as *const c_void, gl::STATIC_DRAW,);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, ptr::null(), );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, (6 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(3, 2, gl::FLOAT, gl::FALSE, 12 * mem::size_of::<GLfloat>() as GLsizei, (10 * mem::size_of::<GLfloat>()) as *const c_void);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Shape{ vao, size:indices.len() as i32,}
    }
}

#[allow(non_snake_case)]
struct Model{ shape:Box<Shape>, transform:Box<Transformation>, shader:Box<Shader>}

#[allow(non_snake_case)]
struct ModelRenderer{camera:Box<Camera>,  models:Vec<Box<Model>>}
impl ModelRenderer{
    pub unsafe fn render(&self){
        for model in &self.models{
            gl::BindVertexArray(model.shape.vao);
            model.shader.bind();
            model.shader.setUniformMatrix("transform", model.transform.getTransform());
            model.shader.setUniformMatrix("projection", self.camera.getProjectionMatrix());
            model.shader.setUniformMatrix("view", self.camera.getViewMatrix());
            gl::DrawElements(gl::TRIANGLES, model.shape.size, gl::UNSIGNED_INT,  0 as *const _);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Grafx Engine").with_inner_size(glutin::dpi::LogicalSize::new(800, 480));
    let context = unsafe {
        let context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
        context.make_current().unwrap()
    };

    gl::load_with(| symbol | context.get_proc_address(symbol) as *const _);

    let vertex_source: &str = r#"
        #version 330 core

        layout (location = 0) in vec3 position;
        layout (location = 1) in vec3 normal;
        layout (location = 2) in vec4 color;
        layout (location = 3) in vec2 text;

        out vec4 colorOut;
        out vec4 normalLight;
        out vec3 fragPosition;

        uniform mat4 transform;
        uniform mat4 view;
        uniform mat4 projection;

        void main(){
            fragPosition=vec3(transform * vec4(position,1.0));
            gl_Position=projection * (view*vec4(fragPosition,1.0));
            normalLight=transform*vec4(normal,1.0);
            colorOut = color;
        }
    "#;

    let fragment_source: &str = r#"
        #version 330 core
        out vec4 FragColor;

        in vec4 colorOut;
        in vec4 normalLight;
        in vec3 fragPosition;

        void main(){
           // Set the fragment color to the color passed from the vertex shader
           FragColor = colorOut;
        }
       "#;
    let shader = unsafe{ Shader::new(vertex_source, fragment_source)};

    let model = Model{
        shape: Box::new(Shape::new(1.0, 1.0, 1.0, Color::Red())),
        transform:Box::new(Transformation::new()),
        shader:Box::new(shader),
    };

    let mut camera = Camera::new(45.0, 800.0, 480.0);
    camera.setPosition(5.0, 5.0, 5.0);

    let mut renderer = ModelRenderer{ camera:Box::new(camera), models: vec![Box::new(model)], };

    unsafe {
        LAST_TIME = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        gl::FrontFace(gl::CW);
        gl::CullFace(gl::BACK);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
    }
    event_loop.run(move | event, _, control_flow| {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent{ event, ..} => match event{
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit},
                WindowEvent::Resized(size) => context.resize(size),
                _ => (),
            }
            _ =>()
        }

        unsafe {
            let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
            DELTA_TIME = current - LAST_TIME;
            LAST_TIME = current;
            gl::ClearColor(0.0, 0.0, 0.2, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            renderer.models[0].transform.rotate(30.0 * DELTA_TIME as f32, 30.0 * DELTA_TIME as f32, 30.0 * DELTA_TIME as f32);
            renderer.models[0].transform.update();
            renderer.render();
        }
        context.swap_buffers().unwrap();
    });
}