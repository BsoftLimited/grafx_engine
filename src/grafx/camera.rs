use crate::grafx::physics::{Vector2, Vector3, Matrix, Matrix4};

pub struct CameraData{ view:Box<Matrix4>, projection:Box<Matrix4> }

impl CameraData{
    pub fn get_view(&self)->&[[f32; 4]; 4]{ self.view.get_data() }
    pub fn get_projection(&self)->&[[f32; 4]; 4]{ self.projection.get_data() }
}

pub struct Frustrum{ width:f32, height:f32, near:f32, far:f32 }
#[allow(dead_code)]
impl Frustrum{
    fn new(width:f32, height:f32, near:f32, far:f32)->Self{
        Frustrum{ width, height, near, far }
    }

    pub fn set_viewport_size(&mut self, width:f32, height:f32){ 
        self.width = width;
        self.height = height
    }

    pub fn set_near(&mut self, near:f32){ self.near = near; }
    pub fn set_far(&mut self, far:f32){ self.far = far; }

    pub fn get_view_port_width(&self)->f32{ self.width }
    pub fn get_view_port_hieght(&self)->f32{ self.height }
}

pub trait Camera{
    fn get_data(&self)->&CameraData;
    fn get_frustum(&self)->&Frustrum;
    fn get_frustum_mut(&mut self)->&mut Frustrum;
    fn update(&mut self);
}

pub struct OrthographicCamera{ data:Box<CameraData>, position:Box<Vector2>, frustrum:Box<Frustrum>}
#[allow(dead_code)]
impl OrthographicCamera{
    pub fn new(port_width:f32, port_hieght:f32)->Self{
        let position = Box::new(Vector2::zero());
        let frustrum = Box::new(Frustrum::new(port_width, port_hieght, 0.0, 0.0));
        let data = CameraData{
            view:Box::new(Matrix4::identity()),
            projection:Box::new(Matrix4::orthogonal_matrix(port_hieght, 0.0, 0.0, port_width, 0.0, 0.0))
        };
        OrthographicCamera{ data:Box::new(data), position, frustrum}
    }

    pub fn get_position(&self)->&Vector2{ self.position.as_ref() }
    pub fn set_position(&mut self, x:f32, y:f32){ self.position.set(x, y); }
    pub fn translate(&mut self, x:f32, y:f32){ self.position.add(x, y); }
}

impl Camera for OrthographicCamera{    
    fn get_frustum(&self) -> &Frustrum { self.frustrum.as_ref() }
    fn get_frustum_mut(&mut self) -> &mut Frustrum { self.frustrum.as_mut() }
    fn get_data(&self) -> &CameraData { self.data.as_ref() }

    fn update(&mut self) {
        self.data.projection = Box::new(Matrix4::orthogonal_matrix(
            self.frustrum.height,
            self.position.get_y(),
            self.position.get_x(),
            self.frustrum.width, 0.0, 0.0));
    }
}

pub struct PerspectiveCamera{ fov:f32, data:Box<CameraData>, position:Box<Vector3>, target:Box<Vector3>, frustrum:Box<Frustrum>}
#[allow(dead_code)]
impl PerspectiveCamera{
    pub fn new(fov:f32, port_width:i32, port_hieght:i32)->Self{
        let position = Box::new(Vector3::zero());
        let target = Box::new(Vector3::zero());
        let frustrum = Box::new(Frustrum::new(port_width as f32, port_hieght as f32, 1.0, 1000.0));
        let view = Matrix4::lookAt_matrix(&position, &target, &Vector3::up());
        let projection = Matrix4::projection_matrix(fov, port_width as f32, port_hieght as f32, 1.0, 1000.0);
        let data = CameraData{ view:Box::new(view), projection:Box::new(projection)};
        PerspectiveCamera{  fov, data:Box::new(data), position, target, frustrum}
    }

    pub fn get_position(&self)->&Vector3{ self.position.as_ref() }
    pub fn set_position(&mut self, x:f32, y:f32, z:f32){ self.position.set(x, y, z); }
    pub fn translate(&mut self, x:f32, y:f32, z:f32){ self.position.add(x, y, z); }
    pub fn look_at(&mut self, x:f32, y:f32, z:f32){ self.target.set(x, y, z); }
}

impl Camera for PerspectiveCamera{
    fn get_frustum(&self) -> &Frustrum { self.frustrum.as_ref() }
    fn get_frustum_mut(&mut self) -> &mut Frustrum { self.frustrum.as_mut() }
    fn get_data(&self) -> &CameraData { self.data.as_ref() }
    fn update(&mut self) {
        self.data.projection = Box::new(Matrix4::projection_matrix(
            self.fov, self.frustrum.width, self.frustrum.height, self.frustrum.near, self.frustrum.far));
        self.data.view = Box::new(Matrix4::lookAt_matrix(self.position.as_ref(), self.target.as_ref(), &Vector3::up()))
    }
}