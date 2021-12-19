

use crate::grafx::physics::Vector3;
use std::ops::Mul;
use std::fmt::{Display, Formatter, Error};

#[allow(non_snake_case)]
pub trait Matrix{
    fn identity()->Self;
    fn zero()->Self;
    fn setValue(&mut self, row:usize, col:usize, value:f32);
    fn getValue(&self, row:usize, col:usize)->f32;
    fn determinant(&self)->f32;
    fn transpose(&mut self)->Self;
    fn coefficient(&mut self)->Self;
    fn inverse(&mut self)->Self;
    fn getSize(&self)->usize;
}

#[allow(non_snake_case)]
pub struct Matrix2{ size: usize, data:[[f32; 2]; 2], }
#[allow(dead_code)]
#[allow(non_snake_case)]
impl Matrix2{
    fn getData(&self)->[[f32; 2]; 2]{ self.data}
}

#[allow(non_snake_case)]
impl Matrix for Matrix2{
    fn identity()->Self{ Matrix2{ size: 2, data:[ [1.0, 0.0],      [0.0, 1.0]]} }
    fn zero()->Self{
        Matrix2{ size: 2, data:[ [0.0, 0.0],      [0.0, 0.0]]}
    }

    fn setValue(&mut self, row: usize, col: usize, value:f32){ self.data[row][col] = value;}
    fn getValue(&self, row: usize, col: usize,)->f32{ self.data[row][col] }

    fn determinant(&self)->f32{
        (self.getValue(0, 0) * self.getValue(1, 1)) - (self.getValue(0, 1) * self.getValue(1, 0))
    }

    fn transpose(&mut self)->Self{
        let a12 = self.getValue(0, 1);
        let a21 = self.getValue(1, 0);

        let mut init = Matrix2::zero();
        init.setValue(0, 0, self.getValue(0, 0));
        init.setValue(0, 1, a21);
        init.setValue(1, 0, a12);
        init.setValue(1, 1, self.getValue(1, 1));
        return init;
    }

    fn coefficient(&mut self) -> Self {
        let mut init = Matrix2::zero();
        init.setValue(0, 0, self.getValue(0, 0));
        init.setValue(0, 1, -self.getValue(1, 0));
        init.setValue(1, 0, -self.getValue(0, 1));
        init.setValue(1, 1, self.getValue(1, 1));
        return init;
    }

    fn inverse(&mut self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }

    fn getSize(&self)->usize{ self.size }
}

impl Mul for &Matrix2{
    type Output = Matrix2;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut init = Matrix2::zero();
        let a00 = self.getValue(0, 0) * rhs.getValue(0, 0) + self.getValue(0, 1) * rhs.getValue(1, 0); 
        let a01 = self.getValue(0, 0) * rhs.getValue(0, 1) + self.getValue(0, 1) * rhs.getValue(1, 1); 

        let a10 = self.getValue(1, 0) * rhs.getValue(0, 0) + self.getValue(1, 1) * rhs.getValue(1, 0); 
        let a11 = self.getValue(1, 0) * rhs.getValue(0, 1) + self.getValue(1, 1) * rhs.getValue(1, 1); 

        init.setValue( 0, 0, a00);      init.setValue( 0, 1, a01);
        init.setValue( 1, 0, a10);      init.setValue( 1, 1, a11);
        return init;
    }
}

impl Mul<f32> for &Matrix2{
    type Output = Matrix2;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut init = Matrix2::zero();
        init.setValue( 0, 0, self.getValue(0, 0) * rhs);    init.setValue( 0, 1, self.getValue(0, 1) * rhs);
        init.setValue( 1, 0, self.getValue(1, 0) * rhs);    init.setValue( 1, 1, self.getValue(1, 1) * rhs);
        return init;
    }
}

impl Display for Matrix2{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}\n{}, {}", self.getValue(0, 0), self.getValue(0, 1), self.getValue(1,0), self.getValue(1,1))
    }
}

#[allow(non_snake_case)]
pub struct Matrix3{ size: usize, data:[[f32; 3]; 3], }
#[allow(non_snake_case)]
impl Matrix for Matrix3{
    fn identity()->Self{
        Matrix3{ size:3, data:[ [1.0, 0.0, 0.0],      [0.0, 1.0, 0.0],     [0.0, 0.0, 1.0]]}
    }

    fn zero()->Self{
        Matrix3{ size:3, data:[ [0.0, 0.0, 0.0],      [0.0, 0.0, 0.0],     [0.0, 0.0, 0.0]]}
    }

    fn setValue(&mut self, row: usize, col: usize, value:f32){
        self.data[row][col] = value;
    }

    fn getValue(&self, row: usize, col: usize,)->f32{
        self.data[row][col]
    }

    fn determinant(&self)->f32{
        let mut a = Matrix2::zero();
        a.setValue( 0, 0, self.getValue(1, 1));     a.setValue( 0, 1, self.getValue(1, 2));
        a.setValue( 1, 0, self.getValue(2, 1));     a.setValue( 0, 1, self.getValue(2, 2));
        let mut value = a.determinant() * self.getValue( 0, 0);

        let mut b = Matrix2::zero();
        b.setValue( 0, 0, self.getValue(1, 0));     b.setValue( 0, 1, self.getValue(1, 2));
        b.setValue( 1, 0, self.getValue(2, 0));     b.setValue( 0, 1, self.getValue(2, 2));
        value += b.determinant() * self.getValue( 0, 1);

        let mut c = Matrix2::zero();
        c.setValue( 0, 0, self.getValue(1, 0));     c.setValue( 0, 1, self.getValue(1, 1));
        c.setValue( 1, 0, self.getValue(2, 0));     c.setValue( 0, 1, self.getValue(2, 1));

        return value + c.determinant() * self.getValue( 0, 2);
    }

    fn transpose(&mut self) -> Self {
        let a01 = self.getValue(1, 0);  let a02 = self.getValue(2, 0);
        let a10 = self.getValue(0, 1);  let a12 = self.getValue(2, 1);
        let a20 = self.getValue(0, 2);  let a21 = self.getValue(1, 2);

        let mut init = Matrix3::zero();
        init.setValue(0, 0, self.getValue(0, 0));  init.setValue(0, 1, a01);   init.setValue(0, 2, a02);
        init.setValue(1, 0, a10);   init.setValue(1, 1, self.getValue(1, 1));  init.setValue(1, 2, a12);
        init.setValue(2, 0, a20);   init.setValue(2, 1, a21);   init.setValue(2, 2, self.getValue(2, 2));
        return init;
    }

    fn coefficient(&mut self) -> Self {
        let mut a00 = Matrix2::zero();
        a00.setValue( 0, 0, self.getValue(1, 1));     a00.setValue( 0, 1, self.getValue(1, 2));
        a00.setValue( 1, 0, self.getValue(2, 1));     a00.setValue( 0, 1, self.getValue(2, 2));

        let mut a01 = Matrix2::zero();
        a01.setValue( 0, 0, self.getValue(1, 0));     a01.setValue( 0, 1, self.getValue(1, 2));
        a01.setValue( 1, 0, self.getValue(2, 0));     a01.setValue( 0, 1, self.getValue(2, 2));

        let mut a02 = Matrix2::zero();
        a02.setValue( 0, 0, self.getValue(1, 0));     a02.setValue( 0, 1, self.getValue(1, 1));
        a02.setValue( 1, 0, self.getValue(2, 0));     a02.setValue( 0, 1, self.getValue(2, 1));

        let mut a10 = Matrix2::zero();
        a10.setValue( 0, 0, self.getValue(0, 1));     a10.setValue( 0, 1, self.getValue(0, 2));
        a10.setValue( 1, 0, self.getValue(2, 1));     a10.setValue( 0, 1, self.getValue(2, 2));

        let mut a11 = Matrix2::zero();
        a11.setValue( 0, 0, self.getValue(0, 0));     a11.setValue( 0, 1, self.getValue(0, 2));
        a11.setValue( 1, 0, self.getValue(2, 0));     a11.setValue( 0, 1, self.getValue(2, 2));

        let mut a12 = Matrix2::zero();
        a12.setValue( 0, 0, self.getValue(0, 0));     a12.setValue( 0, 1, self.getValue(0, 1));
        a12.setValue( 1, 0, self.getValue(2, 0));     a12.setValue( 0, 1, self.getValue(2, 1));

        let mut a20 = Matrix2::zero();
        a20.setValue( 0, 0, self.getValue(0, 1));     a20.setValue( 0, 1, self.getValue(0, 2));
        a20.setValue( 1, 0, self.getValue(1, 1));     a20.setValue( 0, 1, self.getValue(1, 2));

        let mut a21 = Matrix2::zero();
        a21.setValue( 0, 0, self.getValue(0, 0));     a21.setValue( 0, 1, self.getValue(0, 2));
        a21.setValue( 1, 0, self.getValue(1, 0));     a21.setValue( 0, 1, self.getValue(1, 2));

        let mut a22 = Matrix2::zero();
        a22.setValue( 0, 0, self.getValue(0, 0));     a22.setValue( 0, 1, self.getValue(0, 1));
        a22.setValue( 1, 0, self.getValue(1, 0));     a22.setValue( 0, 1, self.getValue(1, 1));
        
        let mut init = Matrix3::zero();
        init.setValue(0, 0, a00.determinant()); init.setValue(0, 1, -a01.determinant()); init.setValue(0, 2, a02.determinant());
        init.setValue(1, 0, -a10.determinant()); init.setValue(1, 1, a11.determinant()); init.setValue(1, 2, -a12.determinant());
        init.setValue(2, 0, a20.determinant()); init.setValue(2, 1, -a21.determinant()); init.setValue(2, 2, a22.determinant());
        return init;
    }

    fn inverse(&mut self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }

    fn getSize(&self)->usize{ self.size }
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
pub struct Matrix4{ size: usize, data:[[f32; 4]; 4], }

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Matrix4{
    pub fn set(&mut self, matrix:&Matrix4){ self.data = matrix.data; }

    pub fn getData(&self)->[[f32; 4]; 4]{ self.data }
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
        let cameraUp = &cameraDirection *  &cameraRight;

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
        Matrix4{ size:4, data:[ [1.0, 0.0, 0.0, 0.0],      [0.0, 1.0, 0.0, 0.0],     [0.0, 0.0, 1.0, 0.0],    [0.0, 0.0, 0.0, 1.0]]}
    }

    fn zero()->Self{
        Matrix4{ size:4, data:[ [0.0, 0.0, 0.0, 0.0],      [0.0, 0.0, 0.0, 0.0],     [0.0, 0.0, 0.0, 0.0],    [0.0, 0.0, 0.0, 0.0]]}
    }

    fn setValue(&mut self, row: usize, col: usize, value:f32){
        self.data[row][col] = value;
    }

    fn getValue(&self, row: usize, col: usize,)->f32{
        self.data[row][col]
    }

    fn determinant(&self)->f32{
        let mut a = Matrix3::zero();
        a.setValue( 0, 0, self.getValue(1, 1));     a.setValue( 0, 1, self.getValue(1, 2));     a.setValue( 0, 2, self.getValue(1, 3));
        a.setValue( 1, 0, self.getValue(2, 1));     a.setValue( 1, 1, self.getValue(2, 2));     a.setValue( 1, 2, self.getValue(2, 3));
        a.setValue( 2, 0, self.getValue(3, 1));     a.setValue( 2, 1, self.getValue(3, 2));     a.setValue( 2, 2, self.getValue(3, 3));
        let mut value = a.determinant() * self.getValue( 0, 0);

        let mut b = Matrix3::zero();
        b.setValue( 0, 0, self.getValue(1, 0));     b.setValue( 0, 1, self.getValue(1, 2));     b.setValue( 0, 2, self.getValue(1, 3));
        b.setValue( 1, 0, self.getValue(2, 0));     b.setValue( 1, 1, self.getValue(2, 2));     b.setValue( 1, 2, self.getValue(2, 3));
        b.setValue( 2, 0, self.getValue(3, 0));     b.setValue( 2, 1, self.getValue(3, 2));     b.setValue( 2, 2, self.getValue(3, 3));
        value -= b.determinant() * self.getValue( 0, 1);

        let mut c = Matrix3::zero();
        c.setValue( 0, 0, self.getValue(1, 0));     c.setValue( 0, 1, self.getValue(1, 1));     c.setValue( 0, 2, self.getValue(1, 3));
        c.setValue( 1, 0, self.getValue(2, 0));     c.setValue( 1, 1, self.getValue(2, 1));     c.setValue( 1, 2, self.getValue(2, 3));
        c.setValue( 2, 0, self.getValue(3, 0));     c.setValue( 2, 1, self.getValue(3, 1));     c.setValue( 2, 2, self.getValue(3, 3));
        value += c.determinant() * self.getValue( 0, 2);

        let mut d = Matrix3::zero();
        d.setValue( 0, 0, self.getValue(1, 0));     d.setValue( 0, 1, self.getValue(1, 1));     d.setValue( 0, 2, self.getValue(1, 2));
        d.setValue( 1, 0, self.getValue(2, 0));     d.setValue( 1, 1, self.getValue(2, 1));     d.setValue( 1, 2, self.getValue(2, 2));
        d.setValue( 2, 0, self.getValue(3, 0));     d.setValue( 2, 1, self.getValue(3, 1));     d.setValue( 2, 2, self.getValue(3, 2));

        return value - d.determinant() * self.getValue( 0, 3);
    }

    fn transpose(&mut self) -> Self {
        let a01 = self.getValue(1, 0);  let a02 = self.getValue(2, 0); let a03 = self.getValue(3, 0);
        let a10 = self.getValue(0, 1);  let a12 = self.getValue(2, 1); let a13 = self.getValue(3, 1);
        let a20 = self.getValue(0, 2);  let a21 = self.getValue(1, 2); let a23 = self.getValue(3, 2);
        let a30 = self.getValue(0, 3);  let a31 = self.getValue(1, 3); let a32 = self.getValue(2, 3);

        let mut init = Matrix4::zero();
        init.setValue(0, 0, self.getValue(0, 0));   init.setValue(0, 1, a01);   init.setValue(0, 2, a02);   init.setValue(0, 3, a03);
        init.setValue(1, 0, a10);   init.setValue(1, 1, self.getValue(1, 1));   init.setValue(1, 2, a12);   init.setValue(1, 3, a13);
        init.setValue(2, 0, a20);   init.setValue(2, 1, a21);   init.setValue(2, 2, self.getValue(2, 2));   init.setValue(2, 3, a23);
        init.setValue(3, 0, a30);   init.setValue(3, 1, a31);   init.setValue(3, 2, a32);   init.setValue(3, 3, self.getValue(3, 3));   
        return init;
    }

    fn coefficient(&mut self) -> Self {
        let mut a00 = Matrix3::zero();
        a00.setValue( 0, 0, self.getValue(1, 1));   a00.setValue( 0, 1, self.getValue(1, 2));   a00.setValue( 0, 2, self.getValue(1, 3));
        a00.setValue( 1, 0, self.getValue(2, 1));   a00.setValue( 1, 1, self.getValue(2, 2));   a00.setValue( 1, 2, self.getValue(2, 3));
        a00.setValue( 2, 0, self.getValue(3, 1));   a00.setValue( 2, 1, self.getValue(3, 2));   a00.setValue( 2, 2, self.getValue(3, 3));

        let mut a01 = Matrix3::zero();
        a01.setValue( 0, 0, self.getValue(1, 0));   a01.setValue( 0, 1, self.getValue(1, 2));   a01.setValue( 0, 2, self.getValue(1, 3));
        a01.setValue( 1, 0, self.getValue(2, 0));   a01.setValue( 1, 1, self.getValue(2, 2));   a01.setValue( 1, 2, self.getValue(2, 3));
        a01.setValue( 2, 0, self.getValue(3, 0));   a01.setValue( 2, 1, self.getValue(3, 2));   a01.setValue( 2, 2, self.getValue(3, 3));

        let mut a02 = Matrix3::zero();
        a02.setValue( 0, 0, self.getValue(1, 0));   a02.setValue( 0, 1, self.getValue(1, 1));   a02.setValue( 0, 2, self.getValue(1, 3));
        a02.setValue( 1, 0, self.getValue(2, 0));   a02.setValue( 1, 1, self.getValue(2, 1));   a02.setValue( 1, 2, self.getValue(2, 3));
        a02.setValue( 2, 0, self.getValue(3, 0));   a02.setValue( 2, 1, self.getValue(3, 1));   a02.setValue( 2, 2, self.getValue(3, 3));

        let mut a03 = Matrix3::zero();
        a03.setValue( 0, 0, self.getValue(1, 0));   a03.setValue( 0, 1, self.getValue(1, 1));   a03.setValue( 0, 2, self.getValue(1, 2));
        a03.setValue( 1, 0, self.getValue(2, 0));   a03.setValue( 1, 1, self.getValue(2, 1));   a03.setValue( 1, 2, self.getValue(2, 2));
        a03.setValue( 2, 0, self.getValue(3, 0));   a03.setValue( 2, 1, self.getValue(3, 1));   a03.setValue( 2, 2, self.getValue(3, 2));

        let mut a10 = Matrix3::zero();
        a10.setValue( 0, 0, self.getValue(0, 1));   a10.setValue( 0, 1, self.getValue(0, 2));   a10.setValue( 0, 2, self.getValue(0, 3));
        a10.setValue( 1, 0, self.getValue(2, 1));   a10.setValue( 1, 1, self.getValue(2, 2));   a10.setValue( 1, 2, self.getValue(2, 3));
        a10.setValue( 2, 0, self.getValue(3, 1));   a10.setValue( 2, 1, self.getValue(3, 2));   a10.setValue( 2, 2, self.getValue(3, 3));

        let mut a11 = Matrix3::zero();
        a11.setValue( 0, 0, self.getValue(0, 0));   a11.setValue( 0, 1, self.getValue(0, 2));   a11.setValue( 0, 2, self.getValue(0, 3));
        a11.setValue( 1, 0, self.getValue(2, 0));   a11.setValue( 1, 1, self.getValue(2, 2));   a11.setValue( 1, 2, self.getValue(2, 3));
        a11.setValue( 2, 0, self.getValue(3, 0));   a11.setValue( 2, 1, self.getValue(3, 2));   a11.setValue( 2, 2, self.getValue(3, 3));

        let mut a12 = Matrix3::zero();
        a12.setValue( 0, 0, self.getValue(0, 0));   a12.setValue( 0, 1, self.getValue(0, 1));   a12.setValue( 0, 2, self.getValue(0, 3));
        a12.setValue( 1, 0, self.getValue(2, 0));   a12.setValue( 1, 1, self.getValue(2, 1));   a12.setValue( 1, 2, self.getValue(2, 3));
        a12.setValue( 2, 0, self.getValue(3, 0));   a12.setValue( 2, 1, self.getValue(3, 1));   a12.setValue( 2, 2, self.getValue(3, 3));

        let mut a13 = Matrix3::zero();
        a13.setValue( 0, 0, self.getValue(0, 0));   a13.setValue( 0, 1, self.getValue(0, 1));   a13.setValue( 0, 2, self.getValue(0, 2));
        a13.setValue( 1, 0, self.getValue(2, 0));   a13.setValue( 1, 1, self.getValue(2, 1));   a13.setValue( 1, 2, self.getValue(2, 2));
        a13.setValue( 2, 0, self.getValue(3, 0));   a13.setValue( 2, 1, self.getValue(3, 1));   a13.setValue( 2, 2, self.getValue(3, 2));

        let mut a20 = Matrix3::zero();
        a20.setValue( 0, 0, self.getValue(0, 1));   a20.setValue( 0, 1, self.getValue(0, 2));   a20.setValue( 0, 2, self.getValue(0, 3));
        a20.setValue( 1, 0, self.getValue(1, 1));   a20.setValue( 1, 1, self.getValue(1, 2));   a20.setValue( 1, 2, self.getValue(1, 3));
        a20.setValue( 2, 0, self.getValue(3, 1));   a20.setValue( 2, 1, self.getValue(3, 2));   a20.setValue( 2, 2, self.getValue(3, 3));

        let mut a21 = Matrix3::zero();
        a21.setValue( 0, 0, self.getValue(0, 0));   a21.setValue( 0, 1, self.getValue(0, 2));   a21.setValue( 0, 2, self.getValue(0, 3));
        a21.setValue( 1, 0, self.getValue(1, 0));   a21.setValue( 1, 1, self.getValue(1, 2));   a21.setValue( 1, 2, self.getValue(1, 3));
        a21.setValue( 2, 0, self.getValue(3, 0));   a21.setValue( 2, 1, self.getValue(3, 2));   a21.setValue( 2, 2, self.getValue(3, 3));

        let mut a22 = Matrix3::zero();
        a22.setValue( 0, 0, self.getValue(0, 0));   a22.setValue( 0, 1, self.getValue(0, 1));   a22.setValue( 0, 2, self.getValue(0, 3));
        a22.setValue( 1, 0, self.getValue(1, 0));   a22.setValue( 1, 1, self.getValue(1, 1));   a22.setValue( 1, 2, self.getValue(1, 3));
        a22.setValue( 2, 0, self.getValue(3, 0));   a22.setValue( 2, 1, self.getValue(3, 1));   a22.setValue( 2, 2, self.getValue(3, 3));

        let mut a23 = Matrix3::zero();
        a23.setValue( 0, 0, self.getValue(0, 0));   a23.setValue( 0, 1, self.getValue(0, 1));   a23.setValue( 0, 2, self.getValue(0, 2));
        a23.setValue( 1, 0, self.getValue(1, 0));   a23.setValue( 1, 1, self.getValue(1, 1));   a23.setValue( 1, 2, self.getValue(1, 2));
        a23.setValue( 2, 0, self.getValue(3, 0));   a23.setValue( 2, 1, self.getValue(3, 1));   a23.setValue( 2, 2, self.getValue(3, 2));

        let mut a30 = Matrix3::zero();
        a30.setValue( 0, 0, self.getValue(0, 1));   a30.setValue( 0, 1, self.getValue(0, 2));   a30.setValue( 0, 2, self.getValue(0, 3));
        a30.setValue( 1, 0, self.getValue(1, 1));   a30.setValue( 1, 1, self.getValue(1, 2));   a30.setValue( 1, 2, self.getValue(1, 3));
        a30.setValue( 2, 0, self.getValue(2, 1));   a30.setValue( 2, 1, self.getValue(2, 2));   a30.setValue( 2, 2, self.getValue(2, 3));

        let mut a31 = Matrix3::zero();
        a31.setValue( 0, 0, self.getValue(0, 0));   a31.setValue( 0, 1, self.getValue(0, 2));   a31.setValue( 0, 2, self.getValue(0, 3));
        a31.setValue( 1, 0, self.getValue(1, 0));   a31.setValue( 1, 1, self.getValue(1, 2));   a31.setValue( 1, 2, self.getValue(1, 3));
        a31.setValue( 2, 0, self.getValue(2, 0));   a31.setValue( 2, 1, self.getValue(2, 2));   a31.setValue( 2, 2, self.getValue(2, 3));

        let mut a32 = Matrix3::zero();
        a32.setValue( 0, 0, self.getValue(0, 0));   a32.setValue( 0, 1, self.getValue(0, 1));   a32.setValue( 0, 2, self.getValue(0, 3));
        a32.setValue( 1, 0, self.getValue(1, 0));   a32.setValue( 1, 1, self.getValue(1, 1));   a32.setValue( 1, 2, self.getValue(1, 3));
        a32.setValue( 2, 0, self.getValue(2, 0));   a32.setValue( 2, 1, self.getValue(2, 1));   a32.setValue( 2, 2, self.getValue(2, 3));

        let mut a33 = Matrix3::zero();
        a33.setValue( 0, 0, self.getValue(0, 0));   a33.setValue( 0, 1, self.getValue(0, 1));   a33.setValue( 0, 2, self.getValue(0, 2));
        a33.setValue( 1, 0, self.getValue(1, 0));   a33.setValue( 1, 1, self.getValue(1, 1));   a33.setValue( 1, 2, self.getValue(1, 2));
        a33.setValue( 2, 0, self.getValue(2, 0));   a33.setValue( 2, 1, self.getValue(2, 1));   a33.setValue( 2, 2, self.getValue(2, 2));
        
        let mut init = Matrix4::zero();
        init.setValue(0, 0, a00.determinant()); init.setValue(0, 1, -a01.determinant()); init.setValue(0, 2, a02.determinant()); init.setValue(0, 3, -a03.determinant());
        init.setValue(1, 0, -a10.determinant()); init.setValue(1, 1, a11.determinant()); init.setValue(1, 2, -a12.determinant()); init.setValue(1, 3, a13.determinant());
        init.setValue(2, 0, a20.determinant()); init.setValue(2, 1, -a21.determinant()); init.setValue(2, 2, a22.determinant()); init.setValue(2, 3, -a23.determinant());
        init.setValue(3, 0, -a30.determinant()); init.setValue(3, 1, a31.determinant()); init.setValue(3, 2, -a32.determinant()); init.setValue(3, 3, a23.determinant());
        return init;
    }

    fn inverse(&mut self) -> Self {
        &self.coefficient().transpose() * (1.0 / self.determinant())
    }
    
    fn getSize(&self) -> usize { self.size }
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

impl Mul<f32> for &Matrix4{
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
pub struct Transformation{ transform:Box<Matrix4>, position:Box<Vector3>, rotation:Box<Vector3>, scale:Box<Vector3>}

#[allow(dead_code)]
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
        let mat = Matrix4::TranslateMatrix(&self.position) * Matrix4::RotationMatrix(&self.rotation) * Matrix4::ScaleMatrix(&self.scale);
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