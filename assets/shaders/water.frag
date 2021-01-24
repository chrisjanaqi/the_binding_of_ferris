#version 450
layout(location = 0) in vec4 position;
layout(location = 0) out vec4 o_Target;

// layout(set = 2, binding = 0) uniform WaterCaustics_diffuse {
//     vec4 diffuse;
// };

// layout(set = 2, binding = 1) uniform WaterCaustics_highlight {
//     vec4 highlight;
// };

layout(set = 2, binding = 0) uniform WaterCaustics_time {
    float time;
};

const vec4 diffuse = vec4(0.13, 0.59, 0.95, 1.0);
const vec4 highlight = vec4(1.0);

const mat2 myt = mat2(.12121212, .13131313, -.13131313, .12121212);
const vec2 mys = vec2(1e4, 1e6);
const vec2 zoom1 = vec2(5.0);
const vec2 zoom2 = vec2(4.95);
const vec2 speed1 = vec2(0.2, -0.3);
const vec2 speed2 = vec2(0.1, 0.5);
const float split = 0.03;
const float sharpness = 3.5;

vec2 rhash(vec2 uv) {
    uv *= myt;
    uv *= mys;
    return fract(fract(uv / mys) * uv);
}

vec3 hash(vec3 p) {
    return fract(
        sin(vec3(
            dot(p, vec3(1.0, 57.0, 113.0)),
            dot(p, vec3(57.0, 113.0, 1.0)),
            dot(p, vec3(113.0, 1.0, 57.0)))
        ) * 43758.5453
    );
}

float voronoi2d(const in vec2 point) {
    vec2 p = floor(point);
    vec2 f = fract(point);
    float res = 0.0;
    for (int j = -1; j <= 1; j++) {
        for (int i = -1; i <= 1; i++) {
            vec2 b = vec2(i, j);
            vec2 r = vec2(b) - f + rhash(p + b);
            res += 1. / pow(dot(r, r), 8.);
        }
    }
    return pow(1. / res, 0.0625);
}

vec4 caustics(const in vec2 pos, const in vec2 offset) {
    float r = voronoi2d(pos + offset + vec2(split, split));
    float g = voronoi2d(pos + offset + vec2(split, -split));
    float b = voronoi2d(pos + offset + vec2(-split, -split));
    float a = voronoi2d(pos + offset);
    return pow(vec4(r, g, b, 1.0), vec4(sharpness));
}

void main() {
    vec4 pos = position / position.w;
    vec4 c1 = caustics(zoom1 * pos.xy, speed1 * time);
    vec4 c2 = caustics(zoom2 * pos.xy, speed2 * time);
    // vec4 c3 = caustics(zoom2 * pos.yx, speed1.yx * time);
    vec4 c = min(c1, c2);

    vec4 color = mix(0.1 * diffuse, highlight, c);
    color.a *= 0.5;
    o_Target = color;
}
