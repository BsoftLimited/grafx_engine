use std::rc::Rc;
use std::cell::RefCell;

pub type Shared<T> = Rc<RefCell<T>>;

pub fn shared<T>(t: T)->Shared<T>{
    return Rc::new(RefCell::new(t));
}

use freetype::freetype_sys::FT_Vector;

pub struct Bearing(pub i32, pub i32);

pub struct Size{ width:i32, height:i32}
impl Size{
    pub fn new(width:i32, height:i32)->Self{
        Size{ width, height}
    }

    pub fn get_width(&self)->i32{ self.width }
    pub fn get_height(&self)->i32{ self.height }
}

pub enum CharacterType{ Normal(u32, Size, Bearing, FT_Vector), Control}

pub struct Character{ ch: char, ch_type: CharacterType, }

impl Character{
    pub fn new(ch:char, texture_id:u32, width:i32, height:i32, x:i32, y:i32, advance:FT_Vector)->Self{
        Character{ ch, ch_type:CharacterType::Normal(texture_id, Size::new(width, height), Bearing(x, y), advance)}
    }

    pub fn control(ch:char)->Self{ Character{ ch, ch_type:CharacterType::Control }}
    pub fn get_character(&self)->char{ return self.ch; }
    pub fn get_type(&self)->&CharacterType{ &self.ch_type }

    pub fn get_texture(&self)->u32{
        if let CharacterType::Normal(texture_id, _, _, _) = self.ch_type {
            return texture_id;
        }
        panic!("Character type variant is of type Escape");
    }

    pub fn get_size(&self)->&Size{
        if let CharacterType::Normal(_, size, _, _) = &self.ch_type {
            return size;
        }
        panic!("Character type variant is of type Escape");
    }

    pub fn get_bearing(&self)->&Bearing{
        if let CharacterType::Normal(_, _, bearing, _) = &self.ch_type {
            return bearing;
        }
        panic!("Character type variant is of type Escape");
    }

    pub fn get_advance(&self)->&FT_Vector{
        if let CharacterType::Normal(_, _, _, advance) = &self.ch_type {
            return advance;
        }
        panic!("Character type variant is of type Escape");
    }
}