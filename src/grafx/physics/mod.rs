mod vector;
pub use vector::{Color, Vector3, Vector2};

mod matrix;
pub use matrix::{Matrix, Matrix2, Matrix3, Matrix4, Transformation2D, Transformation3D};

pub struct Rectangle{
    width:f32, height:f32, rotation:f32,
    lower_left:Box<Vector2>, upper_left:Box<Vector2>, 
    lower_right:Box<Vector2>, upper_right:Box<Vector2>}

impl Rectangle{
    pub fn new(x:f32, y:f32, width:f32, height:f32, rotation:f32)->Self{
        let lower_left = Box::new(Rectangle::rotate(- width / 2.0, - height / 2.0, rotation, x, y));
        let upper_left = Box::new(Rectangle::rotate( - width / 2.0,   height / 2.0, rotation, x, y));
        let lower_right = Box::new(Rectangle::rotate( width / 2.0, - height / 2.0, rotation, x, y));
        let upper_right = Box::new(Rectangle::rotate( width / 2.0,  height / 2.0, rotation, x, y));
        Rectangle{ width, height, rotation, lower_left, upper_left, lower_right, upper_right }
    }

    pub fn get_lower_left(&self)->&Vector2{ self.lower_left.as_ref() }
    pub fn get_lower_right(&self)->&Vector2{ self.lower_right.as_ref() }
    pub fn get_upper_left(&self)->&Vector2{ self.upper_left.as_ref() }
    pub fn get_upper_right(&self)->&Vector2{ self.upper_right.as_ref() }

    pub fn as_array(&self)->[&Vector2; 4]{
        [ self.lower_left.as_ref(), self.lower_right.as_ref(),
        self.upper_left.as_ref(), self.upper_right.as_ref()]
    }

    fn rotate(x:f32, y:f32, rotation:f32, position_x:f32, position_y:f32)->Vector2{
        let rad = rotation.to_radians();
        let init_x = x * f32::cos(rad) - y * f32::sin(rad);
        let init_y = x * f32::sin(rad) + y * f32::cos(rad);
        return Vector2::new( position_x + init_x, position_y + init_y);
    }

    fn is_inside(&self, x:f32, y:f32)->bool{
        false
    }

    pub fn has_collided(&self, rect:&Rectangle)->bool{
        let a_rectangle_axis = [
            self.upper_right.as_ref() - self.upper_left.as_ref(),
            self.upper_right.as_ref() - self.lower_right.as_ref(),
            rect.upper_left.as_ref() - rect.lower_left.as_ref(),
            rect.upper_left.as_ref() - rect.upper_right.as_ref() ];

        for a_axis in a_rectangle_axis{
            if !self.is_axis_collision(rect, a_axis){  return false; }
        }
        return true;
    }
    
    fn generate_scalar(corner: &Vector2, axis:&Vector2)->i32{
        //Using the formula for Vector projection. Take the corner being passed in
        //and project it onto the given Axis
        let a_numerator = (corner.get_x() * axis.get_x()) + (corner.get_y() * axis.get_y());
        let a_denominator = axis.get_x().powi(2) + axis.get_y().powi(2);
        let a_division_result = a_numerator / a_denominator;
        let a_corner_projected = Vector2::new(a_division_result * axis.get_x(), a_division_result * axis.get_y());

        //Now that we have our projected Vector, calculate a scalar of that projection
        //that can be used to more easily do comparisons
        let a_scalar = (axis.get_x() * a_corner_projected.get_x()) + (axis.get_y() * a_corner_projected.get_y());
        return a_scalar as i32;
    }

    fn min_max(values:[i32; 4])->(i32, i32){
        let mut min = values[0];
        let mut max = values[0];
        for value in values{
            if min > value{ min = value; }
            if max < value{ max = value; }
        }
        (min, max)
    }

    fn is_axis_collision(&self, rect:&Rectangle, axis:Vector2)->bool{
        //Project the corners of the Rectangle we are checking on to the Axis and
        //get a scalar value of that project we can then use for comparison
        let a_rectangle_a_scalars = [
            Rectangle::generate_scalar(rect.upper_left.as_ref(), &axis),
            Rectangle::generate_scalar(rect.upper_right.as_ref(), &axis),
            Rectangle::generate_scalar(rect.lower_left.as_ref(), &axis),
            Rectangle::generate_scalar(rect.lower_right.as_ref(), &axis) ];

        let a_rectangle_b_scalars = [
            Rectangle::generate_scalar(self.upper_left.as_ref(), &axis),
            Rectangle::generate_scalar(self.upper_right.as_ref(), &axis),
            Rectangle::generate_scalar(self.lower_left.as_ref(), &axis),
            Rectangle::generate_scalar(self.lower_right.as_ref(), &axis) ];

        //Get the Maximum and Minium Scalar values for each of the Rectangles
        let (a_rectangle_a_minimum, a_rectangle_a_maximum) = Rectangle::min_max(a_rectangle_a_scalars);
        let (a_rectangle_b_minimum, a_rectangle_b_maximum) = Rectangle::min_max(a_rectangle_b_scalars);

        //If we have overlaps between the Rectangles (i.e. Min of B is less than Max of A)
        //then we are detecting a collision between the rectangles on this Axis
        if a_rectangle_b_minimum <= a_rectangle_a_maximum && a_rectangle_b_maximum >= a_rectangle_a_maximum{
            return true;
        }else if a_rectangle_a_minimum <= a_rectangle_b_maximum && a_rectangle_a_maximum >= a_rectangle_b_maximum{
            return true;
        }
        return false;
    }
}

pub trait Collidable<T>{
    fn get_boundary(&self)->T;
    fn is_collided(&self, othher: &dyn Collidable<T>)->bool;
}