use crate::vectors;

use std::fmt::{Debug, Formatter, Display, Error};

use crate::vectors::{Vector, Vector2};

pub struct Matrix2{data:[f32; 4],}
pub struct Matrix3{data:[f32; 9],}
pub struct Matrix4{data:[f32; 16],}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub trait Matrix{
    fn zero()->Self;
    fn identity()->Self;
    fn size(&self)->usize;
    fn get(&self, x:usize, y:usize)->f32;
    fn set(&mut self, x:usize, y:usize, value:f32);
    fn determinant(&self)->f32;
    fn transpose(&mut self)->Self;
    fn cross<T:Matrix>(&mut self, matrix:T)->Self;
    fn cross_with_vector<T:Vector>(&self, vector:T) -> &dyn Vector;
    fn multiply(&self, value:f32)->Self;
    fn inverse(&self)->Self;
}

impl Matrix for Matrix2 {
    fn zero() -> Self { Matrix2{data:[0.0, 0.0, 0.0, 0.0]} }
    fn identity() -> Self { Matrix2{data:[1.0, 0.0, 0.0, 1.0]} }
    fn size(&self) -> usize { 2}
    fn get(&self, x: usize, y: usize) ->f32 { self.data[y * self.row_size() + x] }
    fn set(&mut self, x: usize, y: usize, value: f32) {
        self.data[y * self.row_size() + x] = value;
    }

    fn determinant(&self) -> f32 {
        return (self.get(0, 0) * self.get(1, 1)) - (self.get(0, 1) * self.get(1, 0));
    }

    fn transpose(&mut self) -> Self {
        let mut matrix = Matrix2::zero();
        for i in self.size() {
            for j in self.size() {
                if i != j {
                    matrix.set(j, i, self.get(i, j));
                }else {
                    matrix.set(i, j, self.get(i, j));
                }
            }
        }
        return matrix;
    }

    fn cross<T: Matrix>(&mut self, matrix: T)->Self {
        let mut matrix = Matrix2::zero();
        matrix.set(0, 0, self.get(1, 1));
        matrix.set(0, 1, -self.get(1, 0));
        matrix.set(1, 0, -self.get(0, 1));
        matrix.set(1, 1, self.get(0, 0));
        return matrix;
    }

    fn cross_with_vector<T: Vector>(&self, vector: T) -> &dyn Vector {
        let x = (self.get(0, 0) * vector.get(0)) + (self.get(0, 1) * vector.get(1));
        let y = (self.get(1, 0) * vector.get(0)) + (self.get(1, 1) * vector.get(1));
        return &Vector2::new(x, y);
    }

    fn multiply(&self, value:f32)->Self{
        let mut matrix = Matrix2::zero();
        matrix.set(0, 0, self.get(1, 1) * value);
        matrix.set(0, 1, -self.get(1, 0) * value);
        matrix.set(1, 0, -self.get(0, 1) * value);
        matrix.set(1, 1, self.get(0, 0) * value);
        return matrix;
    }

    fn inverse(&self)->Self{
        return self.coefficent().transpose() * (1/self.determinant());
    }
}

impl Display for Matrix2{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}, {}\n{}, {}", self.get(0, 0), self.get(0, 1), self.get(1,0), self.get(1,1))
    }
}