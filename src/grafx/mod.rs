extern crate glutin;
extern crate gl;

mod resources;

pub mod physics;

mod camera;
use glutin::{ PossiblyCurrent, WindowedContext};
pub use camera::{Camera, ViewPort};

mod shader;
pub use shader::Shader;

mod lights;
pub use lights::{Light, DirectionalLight, PointLight, SpotLight};

mod renderer;
pub use renderer::Renderer;

pub mod models;

use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

static mut DELTA_TIME: f64 = 0.0;
static mut LAST_TIME:f64 = 0.0;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ ControlFlow, EventLoop};
use glutin::window::{WindowBuilder};
use glutin::{ContextBuilder};

#[allow(dead_code)]
#[allow(non_snake_case)]
pub trait Disposable{ fn dispose(&self); }

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct GameWindowDetails{ title:String, width:i32, height: i32}
#[allow(non_snake_case)]
impl GameWindowDetails{
    pub fn new(titl:&str, width:i32, height:i32)-> Self{
        GameWindowDetails{ title:String::from(titl), width, height}
    }
    pub fn getTitle(&self)->&str{ self.title.as_ref() }
    pub fn getWidth(&self)->i32{ self.width }
    pub fn getHeight(&self)->i32{ self.height }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub trait GameWindow{
    fn update(&mut self, delta: f32);
    fn resize(&mut self,width: i32, height:i32);
    unsafe fn render(&self);
}

pub fn init(detail:&GameWindowDetails)->(EventLoop<()>, WindowedContext<PossiblyCurrent>){
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title(detail.getTitle()).with_inner_size(glutin::dpi::LogicalSize::new(detail.getWidth(), detail.getHeight()));
    let context = unsafe {
        let context = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
        context.make_current().unwrap()
    };

    gl::load_with(| symbol | context.get_proc_address(symbol) as *const _);

    unsafe {
        LAST_TIME = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        gl::FrontFace(gl::CW);
        gl::CullFace(gl::BACK);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
    }

    ( event_loop, context)
    
}

pub fn start(win_context: (EventLoop<()>, WindowedContext<PossiblyCurrent>), mut game: Box<dyn GameWindow>){
    let (event_loop, context)  = win_context;
    event_loop.run(move | event, _, control_flow| {   
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent{ event, ..} => match event{
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit},
                WindowEvent::Resized(size) => {
                    unsafe{ gl::Viewport(0, 0, size.width as i32, size.height as i32); }
                    context.resize(size);
                    game.resize(size.width as i32, size.height as i32);
                },
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
            game.render();
            game.update(DELTA_TIME as f32);
        }
        context.swap_buffers().unwrap();
    });
}