#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D u_texture_0;
uniform float u_time;
uniform vec2 u_resolution;
//uniform vec2 u_mouse;

mat2 rotate2D(float angle){
    return mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
}

float hypot(vec2 sides){
    return sqrt(pow(sides.x, 2.0)+pow(sides.y, 2.0));
}

mat2 spherize(vec2 coord, float amount){
    float dist = hypot(coord);
    float b = amount * (2.0 - sqrt(1.0 - 4.0 * pow(dist, 2.0))) - (amount-1.0);
    return mat2(0, b, b, 0);
}

vec4 blur (vec4 preblur, float radius){
    return vec4(1.0);
}

void main(){
    vec2 coord_unmodified = gl_FragCoord.xy / u_resolution;
    vec2 coord = coord_unmodified;

    coord -= 0.5;
    coord = coord * spherize(coord, 0.8);
    coord = coord * rotate2D(-0.2*u_time+0.2*sin(u_time)*sin(0.4+0.3*u_time));
    coord += 0.5;

    coord.x += 0.5*cos(0.2*u_time);
    coord.y += 0.5*sin(0.35*u_time);

    // Adjust scale
    coord /= 1.1;

    // Make texture tileable
    coord = fract(coord.st);

    vec4 image_hex = texture2D(u_texture_0, coord);

    // Reset coord
    coord = coord_unmodified;

    // Create outline circle
    vec4 outline_circle;
    float outline_thickness = 0.01;
    coord -= 0.5;
    coord *= 2.0;
    float hypot = hypot(coord);
    if (hypot < 1.0 && hypot > (1.0 - outline_thickness)) {
        outline_circle = vec4(1.0);
    }

    // Blur


    gl_FragColor = vec4(image_hex.x + outline_circle.x);
}