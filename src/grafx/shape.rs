mod shader;

use gl::types::*;

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