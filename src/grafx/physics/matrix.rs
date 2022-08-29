

use crate::grafx::physics::Vector2;
use crate::grafx::physics::Vector3;
use std::ops::Mul;
use std::fmt::{Display, Formatter, Error};

#[allow(non_snake_case)]
pub trait Matrix{
    fn identity()->Self;
    fn zero()->Self;
    fn set_value(&mut self, row:usize, col:usize, value:f32);
    fn get_value(&self, row:usize, col:usize)->f32;
    fn determinant(&self)->f32;
    fn transpose(&mut self)->Self;
    fn coefficient(&mut self)->Self;
    fn inverse(&mut self)->Self;
    fn get_size(&self)->usize;
}

#[allow(non_snake_case)]
pub struct Matrix2{ size: usize, data:[[f32; 2]; 2], }
#[allow(dead_code)]
#[allow(non_snake_case)]
impl Matrix2{
    fn get_data(&self)->&[[f32; 2]; 2]{ &self.data}
}

#[allow(non_snake_case)]
impl Matrix for Matrix2{
    fn identity()->Self{ Matrix2{ size: 2, data:[ [1.0, 0.0],      [0.0, 1.0]]} }
    fn zero()->Self{
        Matrix2{ size: 2, data:[ [0.0, 0.0],      [0.0, 0.0]]}
    }

    fn set_value(&mut self, row: usize, col: usize, value:f32){ self.data[row][col] = value;}
    fn get_value(&self, row: usize, col: usize,)->f32{ self.data[row][col] }

    fn determinant(&self)->f32{
        (self.get_value(0, 0) * self.get_value(1, 1)) - (self.get_value(0, 1) * self.get_value(1, 0))
    }

    fn transpose(&mut self)->Self{
        let a12 = self.get_value(0, 1);
        let a21 = self.get_value(1, 0);

        let mut init = Matrix2::zero();
        init.set_value(0, 0, self.get_value(0, 0));
        init.set_value(0, 1, a21);
        init.set_value(1, 0, a12);
        init.set_value(1, 1, self.get_value(1, 1));
        return init;
    }

    fn coefficient(&mut self) -> Self {
        let mut init = Matrix2::zero();
        init.set_value(0, 0, self.get_value(0, 0));
        init.set_value(0, 1, -self.get_value(1, 0));
        init.set_value(1, 0, -self.get_value(0, 1));
        init.set_value(1, 1, self.get_value(1, 1));
        return init;
    }

    fn inverse(&mut self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }

    fn get_size(&self)->usize{ self.size }
}

impl Mul for &Matrix2{
    type Output = Matrix2;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut init = Matrix2::zero();
        let a00 = self.get_value(0, 0) * rhs.get_value(0, 0) + self.get_value(0, 1) * rhs.get_value(1, 0); 
        let a01 = self.get_value(0, 0) * rhs.get_value(0, 1) + self.get_value(0, 1) * rhs.get_value(1, 1); 

        let a10 = self.get_value(1, 0) * rhs.get_value(0, 0) + self.get_value(1, 1) * rhs.get_value(1, 0); 
        let a11 = self.get_value(1, 0) * rhs.get_value(0, 1) + self.get_value(1, 1) * rhs.get_value(1, 1); 

        init.set_value( 0, 0, a00);      init.set_value( 0, 1, a01);
        init.set_value( 1, 0, a10);      init.set_value( 1, 1, a11);
        return init;
    }
}

impl Mul<f32> for &Matrix2{
    type Output = Matrix2;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut init = Matrix2::zero();
        init.set_value( 0, 0, self.get_value(0, 0) * rhs);    init.set_value( 0, 1, self.get_value(0, 1) * rhs);
        init.set_value( 1, 0, self.get_value(1, 0) * rhs);    init.set_value( 1, 1, self.get_value(1, 1) * rhs);
        return init;
    }
}

impl Display for Matrix2{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}\n{}, {}", self.get_value(0, 0), self.get_value(0, 1), self.get_value(1,0), self.get_value(1,1))
    }
}

pub struct Matrix3{ size: usize, data:[[f32; 3]; 3], }
impl Matrix3{
    pub fn get_data(&self)->&[[f32; 3]; 3]{ &self.data }

    pub fn rotation_matrix(degree:f32)->Self{
        let mut matrix = Matrix3::identity();
        let radian = f32::to_radians(degree);
        matrix.set_value(0, 0, f32::cos(radian));
        matrix.set_value(0, 1, -f32::sin(radian));
        matrix.set_value(1, 0, f32::sin(radian));
        matrix.set_value(1, 1, f32::cos(radian));
        matrix
    }

    pub fn translate_matrix(vector: &Vector2)->Self{
        let mut matrix = Matrix3::identity();
        matrix.set_value(0, 2, vector.get_x());
        matrix.set_value(1, 2, vector.get_y());
        matrix
    }

    pub fn  scale_matrix(vector: &Vector2)->Self{
        let mut matrix = Matrix3::identity();
        matrix.set_value(0, 0, vector.get_x());
        matrix.set_value(1, 1, vector.get_y());
        matrix
    }
}

#[allow(non_snake_case)]
impl Matrix for Matrix3{
    fn identity()->Self{
        Matrix3{ size:3, data:[ [1.0, 0.0, 0.0],      [0.0, 1.0, 0.0],     [0.0, 0.0, 1.0]]}
    }

    fn zero()->Self{
        Matrix3{ size:3, data:[ [0.0, 0.0, 0.0],      [0.0, 0.0, 0.0],     [0.0, 0.0, 0.0]]}
    }

    fn set_value(&mut self, row: usize, col: usize, value:f32){
        self.data[row][col] = value;
    }

    fn get_value(&self, row: usize, col: usize,)->f32{
        self.data[row][col]
    }

    fn determinant(&self)->f32{
        let mut a = Matrix2::zero();
        a.set_value( 0, 0, self.get_value(1, 1));     a.set_value( 0, 1, self.get_value(1, 2));
        a.set_value( 1, 0, self.get_value(2, 1));     a.set_value( 0, 1, self.get_value(2, 2));
        let mut value = a.determinant() * self.get_value( 0, 0);

        let mut b = Matrix2::zero();
        b.set_value( 0, 0, self.get_value(1, 0));     b.set_value( 0, 1, self.get_value(1, 2));
        b.set_value( 1, 0, self.get_value(2, 0));     b.set_value( 0, 1, self.get_value(2, 2));
        value += b.determinant() * self.get_value( 0, 1);

        let mut c = Matrix2::zero();
        c.set_value( 0, 0, self.get_value(1, 0));     c.set_value( 0, 1, self.get_value(1, 1));
        c.set_value( 1, 0, self.get_value(2, 0));     c.set_value( 0, 1, self.get_value(2, 1));

        return value + c.determinant() * self.get_value( 0, 2);
    }

    fn transpose(&mut self) -> Self {
        let a01 = self.get_value(1, 0);  let a02 = self.get_value(2, 0);
        let a10 = self.get_value(0, 1);  let a12 = self.get_value(2, 1);
        let a20 = self.get_value(0, 2);  let a21 = self.get_value(1, 2);

        let mut init = Matrix3::zero();
        init.set_value(0, 0, self.get_value(0, 0));  init.set_value(0, 1, a01);   init.set_value(0, 2, a02);
        init.set_value(1, 0, a10);   init.set_value(1, 1, self.get_value(1, 1));  init.set_value(1, 2, a12);
        init.set_value(2, 0, a20);   init.set_value(2, 1, a21);   init.set_value(2, 2, self.get_value(2, 2));
        return init;
    }

    fn coefficient(&mut self) -> Self {
        let mut a00 = Matrix2::zero();
        a00.set_value( 0, 0, self.get_value(1, 1));     a00.set_value( 0, 1, self.get_value(1, 2));
        a00.set_value( 1, 0, self.get_value(2, 1));     a00.set_value( 0, 1, self.get_value(2, 2));

        let mut a01 = Matrix2::zero();
        a01.set_value( 0, 0, self.get_value(1, 0));     a01.set_value( 0, 1, self.get_value(1, 2));
        a01.set_value( 1, 0, self.get_value(2, 0));     a01.set_value( 0, 1, self.get_value(2, 2));

        let mut a02 = Matrix2::zero();
        a02.set_value( 0, 0, self.get_value(1, 0));     a02.set_value( 0, 1, self.get_value(1, 1));
        a02.set_value( 1, 0, self.get_value(2, 0));     a02.set_value( 0, 1, self.get_value(2, 1));

        let mut a10 = Matrix2::zero();
        a10.set_value( 0, 0, self.get_value(0, 1));     a10.set_value( 0, 1, self.get_value(0, 2));
        a10.set_value( 1, 0, self.get_value(2, 1));     a10.set_value( 0, 1, self.get_value(2, 2));

        let mut a11 = Matrix2::zero();
        a11.set_value( 0, 0, self.get_value(0, 0));     a11.set_value( 0, 1, self.get_value(0, 2));
        a11.set_value( 1, 0, self.get_value(2, 0));     a11.set_value( 0, 1, self.get_value(2, 2));

        let mut a12 = Matrix2::zero();
        a12.set_value( 0, 0, self.get_value(0, 0));     a12.set_value( 0, 1, self.get_value(0, 1));
        a12.set_value( 1, 0, self.get_value(2, 0));     a12.set_value( 0, 1, self.get_value(2, 1));

        let mut a20 = Matrix2::zero();
        a20.set_value( 0, 0, self.get_value(0, 1));     a20.set_value( 0, 1, self.get_value(0, 2));
        a20.set_value( 1, 0, self.get_value(1, 1));     a20.set_value( 0, 1, self.get_value(1, 2));

        let mut a21 = Matrix2::zero();
        a21.set_value( 0, 0, self.get_value(0, 0));     a21.set_value( 0, 1, self.get_value(0, 2));
        a21.set_value( 1, 0, self.get_value(1, 0));     a21.set_value( 0, 1, self.get_value(1, 2));

        let mut a22 = Matrix2::zero();
        a22.set_value( 0, 0, self.get_value(0, 0));     a22.set_value( 0, 1, self.get_value(0, 1));
        a22.set_value( 1, 0, self.get_value(1, 0));     a22.set_value( 0, 1, self.get_value(1, 1));
        
        let mut init = Matrix3::zero();
        init.set_value(0, 0, a00.determinant()); init.set_value(0, 1, -a01.determinant()); init.set_value(0, 2, a02.determinant());
        init.set_value(1, 0, -a10.determinant()); init.set_value(1, 1, a11.determinant()); init.set_value(1, 2, -a12.determinant());
        init.set_value(2, 0, a20.determinant()); init.set_value(2, 1, -a21.determinant()); init.set_value(2, 2, a22.determinant());
        return init;
    }

    fn inverse(&mut self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }

    fn get_size(&self)->usize{ self.size }
}

impl Mul for &Matrix3{
    type Output = Matrix3;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut init = Matrix3::zero();
        for i in 0..3 {
            for j in 0..3{
                let mut value = 0.0;
                for k in 0..3{
                    value += self.get_value(i, k) * rhs.get_value(k, j);
                }
                init.set_value(i, j, value);
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
                init.set_value(i, j, self.get_value(i, j) * rhs);
            }
        }
        return init;
    }
}

impl Display for Matrix3{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}, {}\n{}, {}, {}\n{}, {}, {}",
               self.get_value(0, 0), self.get_value(0, 1), self.get_value(0, 2),
               self.get_value(1,0), self.get_value(1,1), self.get_value(1,2),
               self.get_value(2,0), self.get_value(2,1), self.get_value(2,2))
    }
}

#[allow(non_snake_case)]
pub struct Matrix4{ size: usize, data:[[f32; 4]; 4], }

#[allow(dead_code)]
impl Matrix4{
    pub fn set(&mut self, matrix:&Matrix4){ self.data = matrix.data; }

    pub fn get_data(&self)->&[[f32; 4]; 4]{ &self.data }
    pub fn x_rotation_matrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_radians(degree);
        matrix.set_value(1, 1, f32::cos(radian));
        matrix.set_value(1, 2, -f32::sin(radian));
        matrix.set_value(2, 1, f32::sin(radian));
        matrix.set_value(2, 2, f32::cos(radian));
        matrix
    }

    pub fn y_rotation_matrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_radians(degree);
        matrix.set_value(0, 0, f32::cos(radian));
        matrix.set_value(0, 2, -f32::sin(radian));
        matrix.set_value(2, 0, f32::sin(radian));
        matrix.set_value(2, 2, f32::cos(radian));
        matrix
    }

    pub fn z_rotation_matrix(degree:f32)->Self{
        let mut matrix = Matrix4::identity();
        let radian = f32::to_radians(degree);
        matrix.set_value(0, 0, f32::cos(radian));
        matrix.set_value(0, 1, -f32::sin(radian));
        matrix.set_value(1, 0, f32::sin(radian));
        matrix.set_value(1, 1, f32::cos(radian));
        matrix
    }

    pub fn rotation_matrix(vector:&Vector3)->Self{
        &Matrix4::x_rotation_matrix(vector.get_x()) * &(&Matrix4::y_rotation_matrix(vector.get_y()) * &Matrix4::z_rotation_matrix(vector.get_z()))
    }

    pub fn translate_matrix(vector: &Vector3)->Self{
        let mut matrix = Matrix4::identity();
        matrix.set_value(0, 3, vector.get_x());
        matrix.set_value(1, 3, vector.get_y());
        matrix.set_value(2, 3, vector.get_z());
        matrix
    }

    pub fn  scale_matrix(vector: &Vector3)->Self{
        let mut matrix = Matrix4::identity();
        matrix.set_value(0, 0, vector.get_x());
        matrix.set_value(1, 1, vector.get_y());
        matrix.set_value(2, 2, vector.get_z());
        matrix
    }

    pub fn projection_matrix(fov:f32, width:f32, hieght:f32, near:f32, far:f32)->Self{
        let mut matrix = Matrix4::identity();
        let ar = width / hieght;
        let angle = 1.0 / f32::tan(f32::to_radians(fov / 2.0));
        let x = far / (far - near);
        matrix.set_value(0, 0, angle / ar);
        matrix.set_value(1, 1, angle);
        matrix.set_value(2, 2, -x);
        matrix.set_value(2, 3, -x * near);
        matrix.set_value(3, 2, -1.0);
        matrix.set_value(3, 3, 0.0);
        return matrix;
    }

    pub fn orthogonal_matrix(top:f32, bottom:f32, left:f32, right:f32, near:f32, far:f32)->Self{
        let mut matrix = Matrix4::identity();
        matrix.set_value(0, 0, 2.0/(right - left));      matrix.set_value(0, 3, -(right + left) / (right - left));
        matrix.set_value(1, 1, 2.0/(top - bottom));      matrix.set_value(1, 3, -(top + bottom) / (top - bottom));
        matrix.set_value(2, 2, -2.0/(far - near));       matrix.set_value(2, 3, -(far + near) / (far - near));

        return matrix;
    }

    pub fn lookAt_matrix(position: &Vector3, target: &Vector3, up: &Vector3)->Self {
        let mut cameraDirection = position - target;
        cameraDirection.normalize();
        let mut cameraRight = up * &cameraDirection;
        cameraRight.normalize();
        let cameraUp = &cameraDirection *  &cameraRight;

        let mut matrixA = Matrix4::identity();
        matrixA.set_value(0, 0, cameraRight.get_x());		matrixA.set_value(0, 1, cameraRight.get_y()); 	matrixA.set_value(0, 2, cameraRight.get_z());
        matrixA.set_value(1, 0, cameraUp.get_x()); 		matrixA.set_value(1, 1, cameraUp.get_y()); 		matrixA.set_value(1, 2, cameraUp.get_z());
        matrixA.set_value(2, 0, cameraDirection.get_x()); matrixA.set_value(2, 1, cameraDirection.get_y()); matrixA.set_value(2, 2, cameraDirection.get_z());

        let mut matrixB = Matrix4::identity();
        matrixB.set_value(0, 3, -position.get_x()); 		matrixB.set_value(1, 3, -position.get_y());		 matrixB.set_value(2, 3, -position.get_z());
        &matrixA * &matrixB
    }
}

#[allow(non_snake_case)]
impl Matrix for Matrix4{
    fn identity()->Self{
        Matrix4{ size:4, data:[ [1.0, 0.0, 0.0, 0.0],      [0.0, 1.0, 0.0, 0.0],     [0.0, 0.0, 1.0, 0.0],    [0.0, 0.0, 0.0, 1.0]]}
    }

    fn zero()->Self{
        Matrix4{ size:4, data:[ [0.0, 0.0, 0.0, 0.0],      [0.0, 0.0, 0.0, 0.0],     [0.0, 0.0, 0.0, 0.0],    [0.0, 0.0, 0.0, 0.0]]}
    }

    fn set_value(&mut self, row: usize, col: usize, value:f32){
        self.data[row][col] = value;
    }

    fn get_value(&self, row: usize, col: usize,)->f32{
        self.data[row][col]
    }

    fn determinant(&self)->f32{
        let mut a = Matrix3::zero();
        a.set_value( 0, 0, self.get_value(1, 1));     a.set_value( 0, 1, self.get_value(1, 2));     a.set_value( 0, 2, self.get_value(1, 3));
        a.set_value( 1, 0, self.get_value(2, 1));     a.set_value( 1, 1, self.get_value(2, 2));     a.set_value( 1, 2, self.get_value(2, 3));
        a.set_value( 2, 0, self.get_value(3, 1));     a.set_value( 2, 1, self.get_value(3, 2));     a.set_value( 2, 2, self.get_value(3, 3));
        let mut value = a.determinant() * self.get_value( 0, 0);

        let mut b = Matrix3::zero();
        b.set_value( 0, 0, self.get_value(1, 0));     b.set_value( 0, 1, self.get_value(1, 2));     b.set_value( 0, 2, self.get_value(1, 3));
        b.set_value( 1, 0, self.get_value(2, 0));     b.set_value( 1, 1, self.get_value(2, 2));     b.set_value( 1, 2, self.get_value(2, 3));
        b.set_value( 2, 0, self.get_value(3, 0));     b.set_value( 2, 1, self.get_value(3, 2));     b.set_value( 2, 2, self.get_value(3, 3));
        value -= b.determinant() * self.get_value( 0, 1);

        let mut c = Matrix3::zero();
        c.set_value( 0, 0, self.get_value(1, 0));     c.set_value( 0, 1, self.get_value(1, 1));     c.set_value( 0, 2, self.get_value(1, 3));
        c.set_value( 1, 0, self.get_value(2, 0));     c.set_value( 1, 1, self.get_value(2, 1));     c.set_value( 1, 2, self.get_value(2, 3));
        c.set_value( 2, 0, self.get_value(3, 0));     c.set_value( 2, 1, self.get_value(3, 1));     c.set_value( 2, 2, self.get_value(3, 3));
        value += c.determinant() * self.get_value( 0, 2);

        let mut d = Matrix3::zero();
        d.set_value( 0, 0, self.get_value(1, 0));     d.set_value( 0, 1, self.get_value(1, 1));     d.set_value( 0, 2, self.get_value(1, 2));
        d.set_value( 1, 0, self.get_value(2, 0));     d.set_value( 1, 1, self.get_value(2, 1));     d.set_value( 1, 2, self.get_value(2, 2));
        d.set_value( 2, 0, self.get_value(3, 0));     d.set_value( 2, 1, self.get_value(3, 1));     d.set_value( 2, 2, self.get_value(3, 2));

        return value - d.determinant() * self.get_value( 0, 3);
    }

    fn transpose(&mut self) -> Self {
        let a01 = self.get_value(1, 0);  let a02 = self.get_value(2, 0); let a03 = self.get_value(3, 0);
        let a10 = self.get_value(0, 1);  let a12 = self.get_value(2, 1); let a13 = self.get_value(3, 1);
        let a20 = self.get_value(0, 2);  let a21 = self.get_value(1, 2); let a23 = self.get_value(3, 2);
        let a30 = self.get_value(0, 3);  let a31 = self.get_value(1, 3); let a32 = self.get_value(2, 3);

        let mut init = Matrix4::zero();
        init.set_value(0, 0, self.get_value(0, 0));   init.set_value(0, 1, a01);   init.set_value(0, 2, a02);   init.set_value(0, 3, a03);
        init.set_value(1, 0, a10);   init.set_value(1, 1, self.get_value(1, 1));   init.set_value(1, 2, a12);   init.set_value(1, 3, a13);
        init.set_value(2, 0, a20);   init.set_value(2, 1, a21);   init.set_value(2, 2, self.get_value(2, 2));   init.set_value(2, 3, a23);
        init.set_value(3, 0, a30);   init.set_value(3, 1, a31);   init.set_value(3, 2, a32);   init.set_value(3, 3, self.get_value(3, 3));   
        return init;
    }

    fn coefficient(&mut self) -> Self {
        let mut a00 = Matrix3::zero();
        a00.set_value( 0, 0, self.get_value(1, 1));   a00.set_value( 0, 1, self.get_value(1, 2));   a00.set_value( 0, 2, self.get_value(1, 3));
        a00.set_value( 1, 0, self.get_value(2, 1));   a00.set_value( 1, 1, self.get_value(2, 2));   a00.set_value( 1, 2, self.get_value(2, 3));
        a00.set_value( 2, 0, self.get_value(3, 1));   a00.set_value( 2, 1, self.get_value(3, 2));   a00.set_value( 2, 2, self.get_value(3, 3));

        let mut a01 = Matrix3::zero();
        a01.set_value( 0, 0, self.get_value(1, 0));   a01.set_value( 0, 1, self.get_value(1, 2));   a01.set_value( 0, 2, self.get_value(1, 3));
        a01.set_value( 1, 0, self.get_value(2, 0));   a01.set_value( 1, 1, self.get_value(2, 2));   a01.set_value( 1, 2, self.get_value(2, 3));
        a01.set_value( 2, 0, self.get_value(3, 0));   a01.set_value( 2, 1, self.get_value(3, 2));   a01.set_value( 2, 2, self.get_value(3, 3));

        let mut a02 = Matrix3::zero();
        a02.set_value( 0, 0, self.get_value(1, 0));   a02.set_value( 0, 1, self.get_value(1, 1));   a02.set_value( 0, 2, self.get_value(1, 3));
        a02.set_value( 1, 0, self.get_value(2, 0));   a02.set_value( 1, 1, self.get_value(2, 1));   a02.set_value( 1, 2, self.get_value(2, 3));
        a02.set_value( 2, 0, self.get_value(3, 0));   a02.set_value( 2, 1, self.get_value(3, 1));   a02.set_value( 2, 2, self.get_value(3, 3));

        let mut a03 = Matrix3::zero();
        a03.set_value( 0, 0, self.get_value(1, 0));   a03.set_value( 0, 1, self.get_value(1, 1));   a03.set_value( 0, 2, self.get_value(1, 2));
        a03.set_value( 1, 0, self.get_value(2, 0));   a03.set_value( 1, 1, self.get_value(2, 1));   a03.set_value( 1, 2, self.get_value(2, 2));
        a03.set_value( 2, 0, self.get_value(3, 0));   a03.set_value( 2, 1, self.get_value(3, 1));   a03.set_value( 2, 2, self.get_value(3, 2));

        let mut a10 = Matrix3::zero();
        a10.set_value( 0, 0, self.get_value(0, 1));   a10.set_value( 0, 1, self.get_value(0, 2));   a10.set_value( 0, 2, self.get_value(0, 3));
        a10.set_value( 1, 0, self.get_value(2, 1));   a10.set_value( 1, 1, self.get_value(2, 2));   a10.set_value( 1, 2, self.get_value(2, 3));
        a10.set_value( 2, 0, self.get_value(3, 1));   a10.set_value( 2, 1, self.get_value(3, 2));   a10.set_value( 2, 2, self.get_value(3, 3));

        let mut a11 = Matrix3::zero();
        a11.set_value( 0, 0, self.get_value(0, 0));   a11.set_value( 0, 1, self.get_value(0, 2));   a11.set_value( 0, 2, self.get_value(0, 3));
        a11.set_value( 1, 0, self.get_value(2, 0));   a11.set_value( 1, 1, self.get_value(2, 2));   a11.set_value( 1, 2, self.get_value(2, 3));
        a11.set_value( 2, 0, self.get_value(3, 0));   a11.set_value( 2, 1, self.get_value(3, 2));   a11.set_value( 2, 2, self.get_value(3, 3));

        let mut a12 = Matrix3::zero();
        a12.set_value( 0, 0, self.get_value(0, 0));   a12.set_value( 0, 1, self.get_value(0, 1));   a12.set_value( 0, 2, self.get_value(0, 3));
        a12.set_value( 1, 0, self.get_value(2, 0));   a12.set_value( 1, 1, self.get_value(2, 1));   a12.set_value( 1, 2, self.get_value(2, 3));
        a12.set_value( 2, 0, self.get_value(3, 0));   a12.set_value( 2, 1, self.get_value(3, 1));   a12.set_value( 2, 2, self.get_value(3, 3));

        let mut a13 = Matrix3::zero();
        a13.set_value( 0, 0, self.get_value(0, 0));   a13.set_value( 0, 1, self.get_value(0, 1));   a13.set_value( 0, 2, self.get_value(0, 2));
        a13.set_value( 1, 0, self.get_value(2, 0));   a13.set_value( 1, 1, self.get_value(2, 1));   a13.set_value( 1, 2, self.get_value(2, 2));
        a13.set_value( 2, 0, self.get_value(3, 0));   a13.set_value( 2, 1, self.get_value(3, 1));   a13.set_value( 2, 2, self.get_value(3, 2));

        let mut a20 = Matrix3::zero();
        a20.set_value( 0, 0, self.get_value(0, 1));   a20.set_value( 0, 1, self.get_value(0, 2));   a20.set_value( 0, 2, self.get_value(0, 3));
        a20.set_value( 1, 0, self.get_value(1, 1));   a20.set_value( 1, 1, self.get_value(1, 2));   a20.set_value( 1, 2, self.get_value(1, 3));
        a20.set_value( 2, 0, self.get_value(3, 1));   a20.set_value( 2, 1, self.get_value(3, 2));   a20.set_value( 2, 2, self.get_value(3, 3));

        let mut a21 = Matrix3::zero();
        a21.set_value( 0, 0, self.get_value(0, 0));   a21.set_value( 0, 1, self.get_value(0, 2));   a21.set_value( 0, 2, self.get_value(0, 3));
        a21.set_value( 1, 0, self.get_value(1, 0));   a21.set_value( 1, 1, self.get_value(1, 2));   a21.set_value( 1, 2, self.get_value(1, 3));
        a21.set_value( 2, 0, self.get_value(3, 0));   a21.set_value( 2, 1, self.get_value(3, 2));   a21.set_value( 2, 2, self.get_value(3, 3));

        let mut a22 = Matrix3::zero();
        a22.set_value( 0, 0, self.get_value(0, 0));   a22.set_value( 0, 1, self.get_value(0, 1));   a22.set_value( 0, 2, self.get_value(0, 3));
        a22.set_value( 1, 0, self.get_value(1, 0));   a22.set_value( 1, 1, self.get_value(1, 1));   a22.set_value( 1, 2, self.get_value(1, 3));
        a22.set_value( 2, 0, self.get_value(3, 0));   a22.set_value( 2, 1, self.get_value(3, 1));   a22.set_value( 2, 2, self.get_value(3, 3));

        let mut a23 = Matrix3::zero();
        a23.set_value( 0, 0, self.get_value(0, 0));   a23.set_value( 0, 1, self.get_value(0, 1));   a23.set_value( 0, 2, self.get_value(0, 2));
        a23.set_value( 1, 0, self.get_value(1, 0));   a23.set_value( 1, 1, self.get_value(1, 1));   a23.set_value( 1, 2, self.get_value(1, 2));
        a23.set_value( 2, 0, self.get_value(3, 0));   a23.set_value( 2, 1, self.get_value(3, 1));   a23.set_value( 2, 2, self.get_value(3, 2));

        let mut a30 = Matrix3::zero();
        a30.set_value( 0, 0, self.get_value(0, 1));   a30.set_value( 0, 1, self.get_value(0, 2));   a30.set_value( 0, 2, self.get_value(0, 3));
        a30.set_value( 1, 0, self.get_value(1, 1));   a30.set_value( 1, 1, self.get_value(1, 2));   a30.set_value( 1, 2, self.get_value(1, 3));
        a30.set_value( 2, 0, self.get_value(2, 1));   a30.set_value( 2, 1, self.get_value(2, 2));   a30.set_value( 2, 2, self.get_value(2, 3));

        let mut a31 = Matrix3::zero();
        a31.set_value( 0, 0, self.get_value(0, 0));   a31.set_value( 0, 1, self.get_value(0, 2));   a31.set_value( 0, 2, self.get_value(0, 3));
        a31.set_value( 1, 0, self.get_value(1, 0));   a31.set_value( 1, 1, self.get_value(1, 2));   a31.set_value( 1, 2, self.get_value(1, 3));
        a31.set_value( 2, 0, self.get_value(2, 0));   a31.set_value( 2, 1, self.get_value(2, 2));   a31.set_value( 2, 2, self.get_value(2, 3));

        let mut a32 = Matrix3::zero();
        a32.set_value( 0, 0, self.get_value(0, 0));   a32.set_value( 0, 1, self.get_value(0, 1));   a32.set_value( 0, 2, self.get_value(0, 3));
        a32.set_value( 1, 0, self.get_value(1, 0));   a32.set_value( 1, 1, self.get_value(1, 1));   a32.set_value( 1, 2, self.get_value(1, 3));
        a32.set_value( 2, 0, self.get_value(2, 0));   a32.set_value( 2, 1, self.get_value(2, 1));   a32.set_value( 2, 2, self.get_value(2, 3));

        let mut a33 = Matrix3::zero();
        a33.set_value( 0, 0, self.get_value(0, 0));   a33.set_value( 0, 1, self.get_value(0, 1));   a33.set_value( 0, 2, self.get_value(0, 2));
        a33.set_value( 1, 0, self.get_value(1, 0));   a33.set_value( 1, 1, self.get_value(1, 1));   a33.set_value( 1, 2, self.get_value(1, 2));
        a33.set_value( 2, 0, self.get_value(2, 0));   a33.set_value( 2, 1, self.get_value(2, 1));   a33.set_value( 2, 2, self.get_value(2, 2));
        
        let mut init = Matrix4::zero();
        init.set_value(0, 0, a00.determinant()); init.set_value(0, 1, -a01.determinant()); init.set_value(0, 2, a02.determinant()); init.set_value(0, 3, -a03.determinant());
        init.set_value(1, 0, -a10.determinant()); init.set_value(1, 1, a11.determinant()); init.set_value(1, 2, -a12.determinant()); init.set_value(1, 3, a13.determinant());
        init.set_value(2, 0, a20.determinant()); init.set_value(2, 1, -a21.determinant()); init.set_value(2, 2, a22.determinant()); init.set_value(2, 3, -a23.determinant());
        init.set_value(3, 0, -a30.determinant()); init.set_value(3, 1, a31.determinant()); init.set_value(3, 2, -a32.determinant()); init.set_value(3, 3, a23.determinant());
        return init;
    }

    fn inverse(&mut self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }
    
    fn get_size(&self) -> usize { self.size }
}

impl Mul for &Matrix4{
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut init = Matrix4::zero();
        for i in 0..4 {
            for j in 0..4{
                let mut value = 0.0;
                for k in 0..4{
                    value += self.get_value(i, k) * rhs.get_value(k, j);
                }
                init.set_value(i, j, value);
            }
        }
        return init;
    }
}

impl Mul<f32> for &Matrix4{
    type Output = Matrix4;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut init = Matrix4::zero();
        for i in 0..4{
            for j in 0..4{
                init.set_value(i, j, self.get_value(i, j) * rhs);
            }
        }
        init
    }
}

impl Display for Matrix4{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}\n{}, {}, {}, {}",
               self.get_value(0, 0), self.get_value(0, 1), self.get_value(0, 2), self.get_value(0, 3),
               self.get_value(1,0), self.get_value(1,1), self.get_value(1,2), self.get_value(1, 3),
               self.get_value(2,0), self.get_value(2,1), self.get_value(2,2), self.get_value(2, 3),
               self.get_value(3,0), self.get_value(3,1), self.get_value(3,2), self.get_value(3, 3),)
    }
}

pub struct Transformation2D{ transform:Box<Matrix3>, position:Box<Vector2>, rotation:f32, scale:Box<Vector2>}
impl Transformation2D{
    pub fn new()->Self{
        Transformation2D{
            transform:Box::new(Matrix3::identity()),
            position:Box::new(Vector2::zero()),
            rotation:0.0,
            scale:Box::new(Vector2::new(1.0, 1.0)),
        }
    }

    pub fn update(&mut self){
        let mat = &Matrix3::translate_matrix(&self.position) * &(&Matrix3::rotation_matrix(self.rotation) * &Matrix3::scale_matrix(&self.scale));
        self.transform.data = mat.data;
    }

    pub fn get_transform_matrix(&self)->&[[f32; 3]; 3]{ &self.transform.data }
    pub fn set_position(&mut self, x:f32, y:f32){ self.position.set(x, y); self.update();}
    pub fn translate(&mut self, x:f32, y:f32){ self.position.add(x, y); self.update();}
    pub fn set_rotation(&mut self, rotation:f32){ self.rotation = rotation; self.update();}
    pub fn rotate(&mut self, rotate:f32){ self.rotation += rotate; self.update();}
    pub fn set_scale(&mut self, x:f32, y:f32){ self.scale.set(x, y); self.update();}
    pub fn scale(&mut self, x:f32, y:f32){ self.scale.set(self.scale.get_x() * x, self.scale.get_y() * y); self.update();}

    pub fn get_position(&self)->&Vector2{ return self.position.as_ref(); }
    pub fn get_rotation(&self)->f32{ return self.rotation; }
    pub fn get_scale(&self)->&Vector2{ return &self.scale.as_ref(); }
}

pub struct Transformation3D{ transform:Box<Matrix4>, position:Box<Vector3>, rotation:Box<Vector3>, scale:Box<Vector3>}
impl Transformation3D{
    pub fn new()->Self{
        Transformation3D{
            transform:Box::new(Matrix4::identity()),
            position:Box::new(Vector3::zero()),
            rotation:Box::new(Vector3::zero()),
            scale:Box::new(Vector3::one()),
        }
    }

    pub fn update(&mut self){
        let mat = &Matrix4::translate_matrix(&self.position) * &(&Matrix4::rotation_matrix(&self.rotation) * &Matrix4::scale_matrix(&self.scale));
        self.transform.data = mat.data;
    }

    pub fn get_transform_matrix(&self)->&[[f32; 4]; 4]{ &self.transform.get_data() }
    pub fn setPosition(&mut self, x:f32, y:f32, z:f32){ self.position.set(x, y, z); }
    pub fn translate(&mut self, x:f32, y:f32, z:f32){ self.position.add(x, y, z); }
    pub fn setRotation(&mut self, x:f32, y:f32, z:f32){ self.rotation.set(x, y, z); }
    pub fn rotate(&mut self, x:f32, y:f32, z:f32){ self.rotation.add(x, y, z); }
    pub fn setScale(&mut self, x:f32, y:f32, z:f32){ self.scale.set(x, y, z); }
    pub fn scale(&mut self, x:f32, y:f32, z:f32){ self.scale.set(self.scale.get_x() * x, self.scale.get_y() * y, self.scale.get_z() * z); }

    pub fn get_position(&self)->&Vector3{ return self.position.as_ref(); }
    pub fn get_rotation(&self)->&Vector3{ return self.rotation.as_ref(); }
    pub fn get_scale(&self)->&Vector3{ return self.scale.as_ref(); }
}
