enum Uniform{
    Integer(&str, i32), Float(&str, f32), Vector2(&str, f32, f32),
    Vector3(&str, f32, f32, f32), Quantanion(&str, f32, f32, f32, f32),
    Matrix(&str, [[f32; 4]; 4]), Array(&str, Uniform, usize), Struct(&str, Vec<Uniform>) 
}

struct MaterialGenerator;

