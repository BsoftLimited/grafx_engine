extern crate gl;
extern crate glutin;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
	
use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::os::raw::c_void;
use std::ffi::CString;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 480;

struct Shader{ program:u32, voa: u32, }

impl Shader{
    fn compile_shader(shader_type: u32, shader_source:&str) -> u32{
        unsafe {
            // Setup shader compilation checks
            let mut success = i32::from(gl::FALSE);
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // -1 to skip trialing null character
    
            // Vertex shader
            let shader = gl::CreateShader(shader_type);
            let c_str_vert = CString::new(shader_source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(shader);
    
            // Check for shader compilation errors
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetShaderInfoLog( shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar,);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",str::from_utf8(&info_log).unwrap());
            }
            shader
        }
    }

    pub fn new(vertex_source:&str, fragment_source:&str) -> Self{
        unsafe {
            let vertex_shader = Shader::compile_shader(gl::VERTEX_SHADER, vertex_source);
            let fragment_shader = Shader::compile_shader(gl::FRAGMENT_SHADER, fragment_source);
    
             // Link Shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            // Setup shader compilation checks
            let mut success = i32::from(gl::FALSE);
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // -1 to skip trialing null character
    
            // Check for linking errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar,);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader{ program:shader_program}
        }
    }
    
    pub fn addUniformMatrix4(&self, nam:&str, matrix):
        unifom = glGetUniformLocation(self.__program, nam)
        # print matrix
        # print str(unifom)+ " " + nam
        glUniformMatrix4fv(unifom, 1, GL_TRUE, numpy.array(matrix.toArray()))

    def _addUniformMatrix3(self, nam, matrix):
        unifom = glGetUniformLocation(self.__program, nam)
        # print str(unifom) + " " + name
        glUniformMatrix3fv(unifom, 1, GL_TRUE, numpy.array(matrix.toArray()))

    def _addUniformQuaternion(self, nam, quaternion):
        unifom = glGetUniformLocation(self.__program, nam)
        # print str(unifom) + " " + name
        if isinstance(quaternion, Quaternion):
            glUniform4f(unifom, quaternion.getX(), quaternion.getY(), quaternion.getZ(), quaternion.getW())
        elif isinstance(quaternion, Color):
            glUniform4f(unifom, quaternion.r, quaternion.g, quaternion.b, quaternion.a)

    def _addUniformVector3(self, nam, vector):
        unifom = glGetUniformLocation(self.__program, nam)
        # print str(unifom) + " " + name
        glUniform3f(unifom, vector.x, vector.y, vector.z)

    def _addUniformVector2(self, nam, vector):
        unifom = glGetUniformLocation(self.__program, nam)
        # print str(unifom) + " " + name
        glUniform2f(unifom, vector.x, vector.y)

    def _addUniformUniformInt(self, nam, value):
        unifom = glGetUniformLocation(self.__program, nam)
        # print str(unifom) + " " + name
        glUniform1i(unifom, value)

    def _addUniformUniformFloat(self, nam, value):
        unifom = glGetUniformLocation(self.__program, nam)
        # print str(unifom) + " " + name
        glUniform1f(unifom, value)

    pub fn use_shader(&self){
        unsafe{
            gl::UseProgram(self.program);
        }
    }
}

pub fn run() {	
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Glutin Triangle").with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    let context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();

    let context = unsafe { context.make_current().unwrap() };
    gl::load_with(| symbol | context.get_proc_address(symbol) as *const _);

    let shader = Shader::new(
        r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            layout (location = 1) in vec3 aColor; // Specify a vertex attribute for color
            out vec3 color;
            void main(){
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                color = aColor; // pass the color along to the fragment shader
            }
        "#,
    
        r#"
            #version 330 core
            out vec4 FragColor;
            in vec3 color;
            void main()
            {
            // Set the fragment color to the color passed from the vertex shader
            FragColor = vec4(color, 1.0);
            }
        "#,);

    let vao = unsafe {
        
        // Set up vao and vbos
        let vertices: [f32; 24] = [
            // top-left
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,

            // top-right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0,

            // bottom-left
            0.0,  0.5, 0.0, 0.0, 0.0, 1.0,

            // bottom-right
            0.0,  0.5, 0.0, 0.0, 0.0, 1.0
        ];

        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, &vertices[0] as *const f32 as *const c_void, gl::STATIC_DRAW,);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as GLsizei, ptr::null(),);

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);


        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Wireframe
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        vao
    };
    
	event_loop.run(move | event, _, control_flow| {
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
					gl::ClearColor(0.0, 0.0, 0.2, 0.0);
					gl::Clear(gl::COLOR_BUFFER_BIT);
					shader.use_shader();
					gl::BindVertexArray(vao);
					gl::DrawArrays(gl::TRIANGLES, 0, 3);
				}
				context.swap_buffers().unwrap();
			}
			_ =>(),
        }
    });
}
