mod text;
use crate::grafx::SpriteRenderer;
use crate::Renderable;
pub use text::Text;

mod container;


pub trait UI :  Renderable<SpriteRenderer>{
    
}

pub trait Clickable : UI{
    
}

pub trait ClickListener{
    fn on_click(&self, ui:dyn Clickable);
}

