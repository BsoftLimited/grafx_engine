pub mod ui;
pub mod sprite;
pub mod models;

use crate::grafx::renderer::ModelRenderer;
use crate::grafx::renderer::SpriteRenderer;
use crate::grafx::physics::Collidable;
use crate::grafx::physics::Rectangle;

pub trait Renderable<T>{
    fn render(&self, renderer:&T);
}

pub trait GameObject2D : Renderable<SpriteRenderer> + Collidable<Rectangle>{

}

pub trait GameObject3D : Renderable<ModelRenderer>{
    
}