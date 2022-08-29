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