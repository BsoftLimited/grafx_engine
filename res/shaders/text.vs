#version 140

in vec4 vertex;
out vec2 TexCoords;

uniform mat4 projection;
uniform mat3 transform;

void main(){
	vec3 position = transform * vec3(vertex.xy, 1.0);
    gl_Position = projection * vec4(position.xy, 0.0, 1.0);
    TexCoords = vertex.zw;
}  
