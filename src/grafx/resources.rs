use crate::grafx::physics::Color;

pub const SIMPLE_VERTEX_SOURCE: &str = r#"
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
            fragPosition = vec3(transform * vec4(position,1.0));
            gl_Position = projection * (view * vec4(fragPosition,1.0));
            normalLight = transform * vec4(normal,1.0);
            colorOut = color;
        }
    "#;

pub const SIMPLE_FRAGMENT_SOURCE: &str = r#"
        #version 330 core

        struct Material{
            vec4 specular; vec4 diffuse;
            vec4 ambient; float shininess;
        };

        struct DirectionalLight{
            vec3 direction;
            vec4 color;
            float ambientStrenght;
            float intensity;
        };

        struct PointLight{
            vec3 position;
            vec4 color;
            float linear;
            float quadratic;
            float intensity;
            float ambientStrenght;
        };

        struct SpotLight{
            vec3 position;
            vec4 color ;
            float intensity;
            vec3 direction;
            float ambientStrenght;
                    
            float linear;
            float quadratic;
            float radius;
            float outerRadius;
        };

        in vec4 colorOut;
        in vec4 normalLight;
        in vec3 fragPosition;

        uniform vec3 cameraPosition;
        uniform Material material;
        uniform DirectionalLight[5] dirLights;
        uniform PointLight[20] pointLights;
        uniform SpotLight[20] spotLights;
        uniform int nDir;
        uniform int nPoint;
        uniform int nSpot;

        vec4 calculateDir(DirectionalLight light ,vec3 norm,vec3 viewDir){
            vec3 lightDir=normalize(-light.direction);
            
            vec4 ambient=(material.ambient * light.color * light.ambientStrenght) * material.diffuse * colorOut ;
            
            vec3 reflectDir = reflect(-lightDir, norm);
            float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
            vec4 specular = material.specular * spec;
            
            float diff= max(dot(vec3(norm),lightDir),0.0);
            vec4 diffuse = light.color * material.diffuse * colorOut *diff;
            
            return (ambient+specular+diffuse) * light.intensity;
        }

        vec4 calculatePoint(PointLight light, vec3 norm,vec3 viewDir){
            vec3 lightDir=normalize(light.position -fragPosition);
            
            float distance=length(light.position-fragPosition);
            float att=1.0/(1.0+(light.linear*distance)+(light.quadratic*(pow(distance,2))));
            
            vec4 ambient=(material.ambient * light.color) * material.diffuse * light.ambientStrenght * colorOut * att;
            
            vec3 reflectDir = reflect(-lightDir, norm);
            float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
            vec4 specular = material.specular * spec * att;
            
            float diff= max(dot(vec3(norm),lightDir),0.0);
            vec4 diffuse = light.color * material.diffuse * diff * colorOut * att;
            
            return (ambient+specular+diffuse) * light.intensity;
        }

        vec4 calculateSpot(SpotLight light, vec3 norm, vec3 viewDir){
            vec3 lightDir=normalize(light.position - fragPosition);
            
            float theta=dot(lightDir,normalize(-light.direction));
            float distance=length(light.position - fragPosition);
            float att=1.0/(1.0+(light.linear * distance)+(light.quadratic*(pow(distance,2))));
            
            if(theta>light.outerRadius){
                float intensity=0;
                if(light.radius > light.outerRadius){
                    float epsilon = light.radius - light.outerRadius;
                    intensity = clamp((theta - light.outerRadius) / epsilon, 0.0, 1.0);
                }
                    
                vec4 ambient=(material.ambient * light.color) * material.diffuse * colorOut ;
                    
                vec3 reflectDir = reflect(-lightDir, norm);
                float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
                vec4 specular = material.specular * spec * intensity * att;
                    
                float diff= max(dot(vec3(norm),lightDir),0.0);
                vec4 diffuse = light.color * material.diffuse * colorOut * diff * intensity * att;
                    
                return (ambient+specular+diffuse) * light.intensity;
                //return vec4(1, 0, 0, 1) * light.color;
            }else{
                return vec4(0);
            }
        }

        void main(){
            
            vec3 norm = normalize(normalLight.xyz);
            vec3 viewDir = normalize(cameraPosition - fragPosition);
            vec4 result = vec4(0);
            for(int i=0; i<nDir; i++){
                result += calculateDir(dirLights[i], norm, viewDir);
            }
            
            for(int i=0; i<nPoint; i++){
                result += calculatePoint(pointLights[i], norm, viewDir);
            }
            
            for(int i=0; i<nSpot; i++){
                result += calculateSpot(spotLights[i], norm, viewDir);
            }
            gl_FragColor = result;
        }
    "#;

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