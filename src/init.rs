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
use std::ops::Mul;
use std::fmt::{Display, Formatter, Error};
use std::time::{SystemTime, UNIX_EPOCH};

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
    fn identity()->Self{
        Matrix2{ rowCount: 2, columnCount:2, data:[ [1.0, 0.0],      [0.0, 1.0]]}
    }

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
        self.coefficient().transpose() * (1.0 / self.determinant())
    }
}

impl Mul for Matrix2{
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

impl Mul<f32> for Matrix2{
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
                            if (k < i) {
                                a = k;
                            } else if (k > i) {
                                a = k - 1;
                            }
                            if (l < j) {
                                b = l;
                            } else if (l > j) {
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

impl Mul for Matrix3{
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

impl Mul<f32> for Matrix3{
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
    pub fn xRotationMatrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_degrees(degree);
        matrix.setValue(1, 1, f32::cos(radian));
        matrix.setValue(1, 2, -f32::sin(radian));
        matrix.setValue(2, 1, f32::sin(radian));
        matrix.setValue(2, 2, f32::cos(radian));
        matrix
    }

    pub fn yRotationMatrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_degrees(degree);
        matrix.setValue(0, 0, f32::cos(radian));
        matrix.setValue(0, 2, -f32::sin(radian));
        matrix.setValue(2, 0, f32::sin(radian));
        matrix.setValue(2, 2, f32::cos(radian));
        matrix
    }

    pub fn zRotationMatrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_degrees(degree);
        matrix.setValue(0, 0, f32::cos(radian));
        matrix.setValue(0, 1, -f32::sin(radian));
        matrix.setValue(1, 0, f32::sin(radian));
        matrix.setValue(1, 1, f32::cos(radian));
        matrix
    }

    pub fn RotationMatrix(x:f32, y:f32, z:f32)->Self{
        Matrix4::xRotationMatrix(x) * Matrix4::yRotationMatrix(y) * Matrix4::zRotationMatrix(z)
    }

    pub fn TranslateMatrix(x:f32, y:f32, z:f32)->Self{
        let mut matrix = Matrix4::identity();
        matrix.setValue(0, 3, x);
        matrix.setValue(1, 3, y);
        matrix.setValue(2, 3, z);
        matrix
    }

    pub fn  ScaleMatrix(x:f32, y:f32, z:f32)->Self{
        let mut matrix = Matrix4::identity();
        matrix.setValue(0, 0, x);
        matrix.setValue(1, 1, y);
        matrix.setValue(2, 2, z);
        matrix
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
                            if (k < i) {
                                a = k;
                            } else if (k > i) {
                                a = k - 1;
                            }
                            if (l < j) {
                                b = l;
                            } else if (l > j) {
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
        return init;
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
struct Transformation{ transform:Matrix4}
#[allow(non_snake_case)]
impl Transformation{

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
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

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
        gl::ProgramUniformMatrix4fv(self.shader_program, uniform, 1, gl::FALSE, std::mem::transmute(&matrix.data));
    }

    pub unsafe  fn bind(&self){
        gl::UseProgram(self.shader_program);
    }
}

struct Box{ vertices:[f32; 288], indices: [u32; 36], }
impl Box{
    pub fn new(width:f32, height:f32, depth:f32, color:Color )->Self{
        Box{
            vertices:[
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
            ],

            indices:[
                2, 1, 0,		0, 3, 2,		//Top
                4, 5, 6,		6, 7, 4,		//Buttom
                10, 9, 8,		8, 11, 10,		//Left
                12, 13, 14,		14, 15, 12,		//Right
                18, 17, 16,		16, 19, 18,		//Front
                20, 21, 22,		22, 23, 20		//Back
            ],
        }
    }
}

#[allow(non_snake_case)]
struct Model{ vao: u32, size: i32}
impl Model{
    pub unsafe fn new()->Self{
        let shape = Box::new(0.5,0.5, 0.5, Color::Blue());
        let vertices = shape.vertices;
        let indices = shape.indices;


        let (mut vbo, mut ebo, mut vao) = (0, 0, 0);
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

        Model{vao, size: indices.len() as i32,}
    }

    pub unsafe  fn draw(&self){
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.size, gl::UNSIGNED_INT,  0 as *const _);
    }
}


static mut deltaTime: f64 = 0.0;
static mut lastTime:f64 = 0.0;
fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Grafx Engine").with_inner_size(glutin::dpi::LogicalSize::new(800, 480));
    let context = unsafe {
        let context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
        context.make_current().unwrap()
    };

    gl::load_with(| symbol | context.get_proc_address(symbol) as *const _);

    let model = unsafe{ Model::new()};

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

    unsafe {lastTime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(); }
    event_loop.run( move | event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent{ event, ..} => match event{
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => context.resize(size),
                _ => (),
            },
            Event::RedrawRequested(_) =>{
                unsafe {
                    let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                    deltaTime = current - lastTime;
                    lastTime = current;

                    gl::ClearColor(0.0, 0.0, 0.2, 0.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    shader.bind();
                    shader.setUniformMatrix("transform", &Matrix4::RotationMatrix(30.0, 3.0, 3.0));
                    shader.setUniformMatrix("projection", &Matrix4::identity());
                    shader.setUniformMatrix("view", &Matrix4::identity());
                    model.draw();
                    println!("{}", deltaTime);
                }
                context.swap_buffers().unwrap();
            }
            _ => window.red,
        }
    });
}