#version 450
layout(location = 0) in vec4 position;
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 1) uniform WaterCaustics_diffuse {
    vec4 diffuse;
};

layout(set = 1, binding = 2) uniform WaterCaustics_highlight {
    vec4 highlight;
};

layout(set = 1, binding = 3) uniform WaterCaustics_time {
    float time;
};

vec3 mod289(vec3 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec2 mod289(vec2 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec3 permute(vec3 x) {
    return mod289((x * 34.0 + 1.0) * x);
}

vec3 taylorInvSqrt(vec3 r) {
    return 1.79284291400159 - 0.85373472095314 * r;
}

float snoise(vec2 v) {
    // NOTE: OUTPUT IS IN [-1, 1] WITH MEDIAN 0
    const vec4 C = vec4(
        0.211324865405187,  // (3.0-sqrt(3.0))/6.0
        0.366025403784439,  // 0.5*(sqrt(3.0)-1.0)
        -0.577350269189626,  // -1.0 + 2.0 * C.x
        0.024390243902439
    ); // 1.0 / 41.0
    
    // First corner
    vec2 i = floor(v + dot(v, C.yy));
    vec2 x0 = v - i + dot(i, C.xx);
    
    // Other corners
    vec2 i1;
    i1.x = step(x0.y, x0.x);
    i1.y = 1.0 - i1.x;
    
    // x1 = x0 - i1  + 1.0 * C.xx;
    // x2 = x0 - 1.0 + 2.0 * C.xx;
    vec2 x1 = x0 + C.xx - i1;
    vec2 x2 = x0 + C.zz;
    
    // Permutations
    i = mod289(i); // Avoid truncation effects in permutation
    vec3 p = permute(permute(i.y + vec3(0.0, i1.y, 1.0)) + i.x + vec3(0.0, i1.x, 1.0));
    
    vec3 m = max(0.5 - vec3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0);
    m = pow(m, vec3(4));
    
    // Gradients: 41 points uniformly over a line, mapped onto a diamond.
    // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
    vec3 x = 2.0 * fract(p * C.www) - 1.0;
    vec3 h = abs(x) - 0.5;
    vec3 ox = floor(x + 0.5);
    vec3 a0 = x - ox;
    
    // Normalise gradients implicitly by scaling m
    m *= taylorInvSqrt(a0 * a0 + h * h);
    
    // Compute final noise value at P
    vec3 g;
    g.x = a0.x * x0.x + h.x * x0.y;
    g.y = a0.y * x1.x + h.y * x1.y;
    g.z = a0.z * x2.x + h.z * x2.y;
    return 130.0 * dot(m, g);
}

float caustic(vec2 pos, vec2 offset, float freq) {
  return pow(1-abs(snoise(offset+pos*freq)), 2.0);
}

void main() {
    vec4 pos = position / position.w;
    float alpha = 1.0;
    float base = 2.0;
    for (int i= 0; i < 3; i++) {
        float freq = pow(base, i+1);
        vec2 offset = vec2(snoise(vec2(sin(0.01 * time * pow(i+1, 3.0)))), snoise(vec2(cos(0.01 * time * pow(i+1, 3.1)))));
        alpha *= pow(caustic(pos.xy, offset, freq), freq);
    }
    vec4 color = mix(diffuse, highlight, alpha);
    color.a *= 0.8;
    o_Target = color;
}
