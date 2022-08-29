use crate::grafx::physics::Color;

#[allow(non_snake_case)]
pub fn getBoxVertices(width:f32, height:f32, depth:f32, color:Color )->([f32; 288], [u32; 36]){
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
        (vertices, indices)
    }