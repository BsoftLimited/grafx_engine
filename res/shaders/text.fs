#version 140
in vec2 TexCoords;
out vec4 color;

uniform sampler2D text;
uniform vec4 textColor;

void main(){
    //vec4 sampled = vec4(1.0, 1.0, texture(text, TexCoords).r);
    //color =  vec4(1.0);
    float diffuse = texture(text, TexCoords).x;
    //color =  vec4( textColor.xyz * diffuse.x, diffuse.y);
    color = vec4(textColor.xyz * diffuse, diffuse);
}  