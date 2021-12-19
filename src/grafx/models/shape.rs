#[allow(non_snake_case)]
pub struct Shape{ vao: u32, size: i32,}

#[allow(non_snake_case)]
impl Shape{
    pub fn getVertexArrayBuffer(&self)->u32{ self.vao}
    pub fn getIndexCount(&self)->i32{ self.size }
}