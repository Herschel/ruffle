#version 450

layout(location=0) in vec4 frag_color;

// Push constants: matrix + color
layout(push_constant) uniform PushConstants {
    mat4 world_matrix;
    vec4 mult_color;
    vec4 add_color;
};

layout(location=0) out vec4 out_color;

void main() {
    out_color = mult_color * frag_color + add_color;
}