
use crate::grafx::objects::ui::UI;
use crate::grafx::renderer::SpriteRenderer;
use crate::Renderer;
use crate::Renderable;
use std::convert::TryInto;
use crate::grafx::materials::Shader;
use crate::grafx::physics::Transformation2D;
use crate::Disposable;
use gl::types::GLsizei;
use gl::types::GLfloat;
use crate::grafx::physics::Color;
use std::ffi::c_void;
use std::{ mem, ptr};
use freetype::face::LoadFlag;
use freetype::Library;
use crate::grafx::utils::{ Character, CharacterType };

static mut TEXT_SHADER:Option<Box<Shader>> = None;

pub struct Text{
    voa:u32, vbo:u32, text:String, font:String, font_size:u32, characters:Box<Vec<Character>>, 
    color:Box<Color>, transform:Box<Transformation2D>
}

#[allow(dead_code)]
impl Text{
    pub fn new(text:&str)->Self{
        let (voa, vbo) = unsafe{
            let mut voa = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut voa);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(voa);
            if let None = TEXT_SHADER {
                let init = Box::new(Shader::text());
                init.bind();
                TEXT_SHADER = Some(init);
            }else if let Some(shader) = &TEXT_SHADER {
                shader.bind();
            }
            (voa, vbo)
        };

        let font = "/home/bsoft/Projects/text_example_rs/target/debug/DroidSansMono.ttf";
        let characters = Text::get_characters(font, text, 16);
        unsafe{
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 4 * 6 * mem::size_of::<GLfloat>() as isize, ptr::null(), gl::DYNAMIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Text{
            voa, vbo, text:String::from(text), font:String::from(font),
            font_size:16, characters, color:Box::new(Color::white()),
            transform:Box::new(Transformation2D::new())
        }
    }

    fn get_characters(font:&str, text:&str, size:u32)->Box<Vec<Character>>{
        let mut characters = Vec::new();
        let lib = Library::init().unwrap();
        let face = lib.new_face(font, 0).unwrap();
        face.set_pixel_sizes(size, 0).unwrap();

        unsafe{ gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1); }
        for i in 0..text.len(){
            let ch = text.chars().nth(i).unwrap();
            if ch.is_control(){
                characters.push(Character::control(ch));
            }else{
                face.load_char(ch as usize, LoadFlag::RENDER).unwrap();
                let glyph = face.glyph();
                unsafe{
                    let mut texture = 0;
                    gl::GenTextures(1,&mut texture);
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED.try_into().unwrap(), glyph.bitmap().width(), glyph.bitmap().rows(), 0,  gl::RED, gl::UNSIGNED_BYTE, glyph.bitmap().raw().buffer as *const c_void);
                    characters.push(Character::new(ch, texture, glyph.bitmap().width(), glyph.bitmap().rows(), glyph.bitmap_left(), glyph.bitmap_top(), glyph.advance()));
                }
            }
        }
        unsafe{ gl::BindTexture(gl::TEXTURE_2D, 0); }
        drop(lib);
        drop(face);
        return Box::new(characters);
    }

    pub fn set_text(&mut self, data:&str){
        if !self.text.eq(data){
            self.text = String::from(data);
            self.dispose();
            self.characters = Text::get_characters(&self.font, &self.text, self.font_size);
        }
    }
    
    pub fn set_font_size(&mut self, size:u32){
        self.font_size = size;
        self.dispose();
        self.characters = Text::get_characters(&self.font, &self.text, self.font_size);
    }
    
    pub fn set_color(&mut self, red:f32, green:f32, blue:f32, alpha:f32){
        self.color = Box::new(Color{red, green, blue, alpha});
    }

    pub fn get_text(&self)->&str{ return self.text.as_ref(); }
    pub fn get_font_size(&self)->u32{ return self.font_size; }
    pub fn get_width(&self)->f32{
        let mut width:f32 = 0.0;
        let mut init:f32 = 0.0;
        for ch in self.characters.as_ref(){
            if let CharacterType::Control = ch.get_type(){
                if ch.get_character() == '\n'{
                    width = if init > width { init } else { width };
                    init = 0.0;
                }
            }else{
                init += (ch.get_advance().x >> 6) as f32;
            }
        }
        return if init > width { init } else { width };
    }

    pub fn get_height(&self)->f32{
        let mut height:f32 = 0.0;
        let mut init:f32 = 0.0;
        for ch in self.characters.as_ref(){
            if let CharacterType::Control = ch.get_type(){
                if ch.get_character() == '\n'{
                    height += init;
                    init = 0.0;
                }
            }else{
                init = if init > ch.get_size().get_height() as f32 { init } else { ch.get_size().get_height() as f32 };
            }
        }
        return height + init;
    }

    pub fn get_transform(&mut self)->&mut Transformation2D{
        &mut self.transform
    }
} 

impl Renderable<SpriteRenderer> for Text{
    fn render(&self, renderer: &SpriteRenderer) {
        unsafe{
            gl::BindVertexArray(self.voa);
            gl::ActiveTexture(gl::TEXTURE0);
            if let Some(shader) = &TEXT_SHADER {
                shader.bind();
                shader.set_uniform_matrix4("projection", renderer.get_projection());
                shader.set_uniform_matrix3("transform", self.transform.get_transform_matrix());
                shader.set_uniform_color("textColor", &self.color);
            }
        }

        let mut x = - self.get_width()/2.0;
        let mut y = - self.get_height() /2.0;
        for ch in self.characters.as_ref(){
            if let CharacterType::Control = ch.get_type(){
            }

            let xpos = x + ch.get_bearing().0 as f32;
            let ypos = y - (ch.get_size().get_height() - ch.get_bearing().1) as f32;
            let width = ch.get_size().get_width() as f32;
            let height = ch.get_size().get_height() as f32;

            let vertices = [           
                [ xpos + width, ypos,             1.0, 1.0 ],
                [ xpos,         ypos,             0.0, 1.0 ],
                [ xpos,         ypos + height,    0.0, 0.0 ], 

                [ xpos + width, ypos,             1.0, 1.0 ],
                [ xpos,         ypos + height,    0.0, 0.0 ],
                [ xpos + width, ypos + height,    1.0, 0.0 ]           
            ];

            unsafe{
                gl::BindTexture(gl::TEXTURE_2D, ch.get_texture());
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl::BufferSubData(gl::ARRAY_BUFFER, 0, 24 * mem::size_of::<GLfloat>() as isize, &vertices[0] as *const f32 as *const c_void); // be sure to use glBufferSubData and not glBufferData

                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }

            x += (ch.get_advance().x >> 6) as f32;
            //y += (ch.get_advance().y >> 6) as f32;
        }
        unsafe{
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl UI for Text{
    
}

impl Disposable for Text{
    fn dispose(&mut self) {
        if !self.characters.is_empty(){
            let ch = self.characters[0].get_texture();
            unsafe{
                gl::BindVertexArray(self.voa);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                gl::DeleteTextures(1, &ch);
                gl::BindVertexArray(0);
            }
            self.characters.clear();
        }
    }
}