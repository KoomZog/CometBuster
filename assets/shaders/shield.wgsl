// TODO
// Improve fullness animation and make it take a float from Bevy

struct Time { value: f32;};
struct ShieldColor { value: i32;};
struct TimeSinceActivation { value: f32;};
struct TimeSinceDeactivation { value: f32;};
struct DeactivationFlash { value: i32;};
struct TimeSinceCollision { value: f32;};
struct CollisionAngle { value: f32;};

struct FragmentInput {
    [[location(0)]] pos: vec2<f32>;
};

[[group(1), binding(0)]] var<uniform> in_time: Time;
[[group(1), binding(1)]] var<uniform> in_color: ShieldColor;
[[group(1), binding(2)]] var<uniform> in_time_since_activation: TimeSinceActivation;
[[group(1), binding(3)]] var<uniform> in_time_since_deactivation: TimeSinceDeactivation;
[[group(1), binding(4)]] var<uniform> in_time_since_collision: TimeSinceCollision;
[[group(1), binding(5)]] var<uniform> in_collision_angle: CollisionAngle;
[[group(1), binding(6)]] var<uniform> in_deactivation_flash: DeactivationFlash;
[[group(1), binding(7)]] var texture_gradient: texture_2d<f32>;
[[group(1), binding(8)]] var texture_sampler_gradient: sampler;

fn hypot(a: f32, b: f32) -> f32 {
    return sqrt(pow(a, 2.0) + pow(b, 2.0));
}

fn spherize(uv: vec2<f32>, amount: f32) -> vec2<f32> {
    let dist = hypot(uv.x, uv.y);
    let b = amount * (2.0 - sqrt(1.0 - 4.0 * pow(dist, 2.0))) - (amount - 1.0);
    return uv * mat2x2<f32>(0.0, b, b, 0.0);
}

fn rotate2D(angle: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(angle), -sin(angle), sin(angle), cos(angle));
}

fn pulse(time: f32, min: f32, max: f32, frequency: f32) -> f32 {
    let pi = 3.14159265358979;
    return ( 0.5 * ( max - min ) * sin( time * frequency * pi) + (( min + max ) / 2.0 ));
}

fn tile(uv: vec2<f32>, zoom: f32) -> vec2<f32> {
    let uv_zoomed = uv * zoom;
    return fract(uv_zoomed);
}

fn hex_base(hex_line_thickness: f32, uv: vec2<f32>, scale_x: f32, scale_y: f32) -> f32 {
    let pi = 3.14159265358979;
    let tau = 2.0 * pi;

    let hex_size = 1.0 - hex_line_thickness / 2.0;
    let r = tau / 6.0; // number of N-gon sides
    let a = atan(uv.x/uv.y)+pi;
    let d = cos(floor(.5+a/r)*r-a)*hypot(uv.x, uv.y)/0.43; // TODO: find where the 0.43 factor actually comes from

    // Center hex outline
    var pixel = 0.0;
    if ( d > hex_size && d < (hex_size + hex_line_thickness)) {
        pixel = 1.0;
    }

    // Lines going from the points, away from the center of the hex - TODO: Find exact thickness (Y) scaling, 4.65 is empirical.
    var uv_rotated = uv;
    for (var i:i32 = 0; i<3; i = i+1){
        if ((uv_rotated.x > (scale_x - 1.0) || uv_rotated.x < (1.0 - scale_x)) && abs(uv_rotated.y) < hex_line_thickness / 4.65) {
            pixel = 1.0;
        }
    uv_rotated = uv_rotated * rotate2D(tau / 6.0);
    }

    return pixel;
}

[[stage(fragment)]]
fn fragment(in_uv: FragmentInput) -> [[location(0)]] vec4<f32> {
    // Basics
    let quad_size = vec2<f32>(100.0);
    var uv = (in_uv.pos / quad_size);
    let pi = 3.14159265358979;
    let tau = 2.0 * pi;

    // -- FROM BEVY --
    let time = in_time.value;
    let time_since_activation = in_time_since_activation.value;
    let time_since_deactivation = in_time_since_deactivation.value;
    let ring_deactivation_flash = in_deactivation_flash.value;
    let time_since_collision = in_time_since_collision.value;
    let collision_angle = in_collision_angle.value;

    // -- SETTINGS --
    // General settings
    let diameter = 0.8;
    let pulse_frequency = 1.0;

    // Ring settings
    let ring_activation_end = 0.05;
    let ring_deactivation_start = 0.3;
    let ring_deactivation_end = 0.6;
    let ring_deactivation_flash_start = 0.5;
    let ring_deactivation_flash_time = 0.2;
    let ring_deactivation_flash_intensity = 0.8;
    let flash = 1.0 / ring_deactivation_flash_time * (time_since_deactivation - ring_deactivation_flash_start);

    // Hex pattern settings
    let hex_size = 0.5;
    let hex_spherize_level = 0.8;
    let hex_activation_start = 0.02;
    let hex_activation_end = 0.15;
    let hex_deactivation_end = 0.4;
    //https://www.desmos.com/calculator/xykhidbkbg
    let hex_activation_factor = min(max( (time_since_activation - hex_activation_start) / (hex_activation_end - hex_activation_start), 0.0), 1.0);
    var hex_deactivation_factor = min(max( (time_since_deactivation - hex_deactivation_end) / -hex_deactivation_end, 0.0), 1.0);
    let hex_base_line_thickness = pulse(time, 0.08, 0.12, pulse_frequency) * hex_activation_factor * hex_deactivation_factor;
    let hex_glow_line_thickness = pulse(time, 0.2, 0.3, pulse_frequency) * hex_activation_factor * hex_deactivation_factor;

    // Blur settings
    let blur_quality_int = 2;
    let hex_base_blur_size = pulse(time, 0.01, 0.015, pulse_frequency);
    let hex_glow_blur_size = pulse(time, 0.06, 0.10, pulse_frequency);


    // -- COLLISION SHOCKWAVE --
    let shockwave_speed = 6.0;
    let shockwave_width = 0.3;
    let shockwave_intensity = 1.0;
    var collision_uv = uv * rotate2D(collision_angle);
    collision_uv = collision_uv * 2.0 / diameter + vec2<f32>(0.0, -1.0); // Not sure about ordering. Try different diameters.
    var collision_hypot = hypot(collision_uv.x, collision_uv.y) + shockwave_width - time_since_collision * shockwave_speed;
    var shockwave_factor = 1.0;
    if (collision_hypot > 0.0 * shockwave_width && collision_hypot < 1.0 * shockwave_width){
        shockwave_factor = 1.0 + shockwave_intensity / 2.0 * (1.0 - cos(pi * (2.0 / shockwave_width * collision_hypot + 2.0 * shockwave_speed)));
    }


    // -- RING --
    let ring_activation_factor = min(max( time_since_activation / ring_activation_end, 0.0), 1.0);
    var ring_deactivation_factor = 1.0 - min(max( (time_since_deactivation - ring_deactivation_start) / (ring_deactivation_end - ring_deactivation_start), 0.0), 1.0);
    if (ring_deactivation_flash > 0) {
        ring_deactivation_factor = max( ring_deactivation_factor, min(flash, -flash) + ring_deactivation_flash_intensity);
    }
    let ring_uv = uv * 2.0;
    let ring_hypot = hypot(ring_uv.x, ring_uv.y);
    var outline_thickness = diameter * 0.005 * shockwave_factor;
    let inner_glow_width = diameter * pulse(time, 0.4, 0.5, 0.8) * shockwave_factor;
    let outer_glow_width = diameter * 0.12 * shockwave_factor;
    var image_outline = 0.0;
    var image_inner_glow = 0.0;
    var image_outer_glow = 0.0;

    // Outline    
    if (ring_hypot < diameter && ring_hypot > (diameter - outline_thickness)) {
        image_outline = 1.0;
    }
    // Inner glow
    if (ring_hypot < (diameter - outline_thickness) && ring_hypot > (diameter - outline_thickness - inner_glow_width)) {
        let inner_glow = 1.0 - (ring_hypot - diameter + outline_thickness) / -inner_glow_width;
        image_inner_glow = pow(inner_glow, 2.5);
    }
    // Outer glow
    if (ring_hypot > diameter && ring_hypot < (diameter + outer_glow_width)) {
        let outer_glow = 1.0 - (ring_hypot - diameter) / outer_glow_width;
        image_outer_glow = pow(outer_glow, 1.6);
    }
    let ring_bw = (0.0 + image_outline + image_inner_glow + image_outer_glow) * ring_activation_factor * ring_deactivation_factor;


    // -- HEX PATTERN --
    var hex_uv = uv;
    hex_uv = hex_uv / diameter;
    hex_uv = spherize(hex_uv, hex_spherize_level);

    // Animate hex pattern rotation and position
    hex_uv = hex_uv * rotate2D( -0.2 * time + 0.2 * sin(time) * sin(0.4 + 0.3 * time));
    hex_uv.x = hex_uv.x + 0.5*cos(0.2 * time);
    hex_uv.y = hex_uv.y + 0.5*sin(0.35 * time);

    // Scale the UV for easy hex tiling
    let scale_x = 1.0 + sin( pi / 6.0 ); // In X, we set the scale to one point-to point length + the length of one side of the hex
    let scale_y = cos( pi / 6.0 ); // In Y, we set the scale to one side-to-side length
    hex_uv.x = hex_uv.x / scale_x;
    hex_uv.y = hex_uv.y / scale_y; 

    // Tile the UV
    hex_uv = tile(hex_uv, 1.0 / hex_size);
    hex_uv = hex_uv % vec2<f32>(1.0);
    hex_uv = hex_uv - vec2<f32>(0.5);

    // Scale the UV back
    hex_uv.x = hex_uv.x * scale_x;
    hex_uv.y = hex_uv.y * scale_y;

    // Blurred base hex
    var blur_brightness_sum = 0.0;
    var blur_weighting_sum = 0.0;
    let blur_quality = f32(blur_quality_int);
    for(var i: f32 = -1.0; i < 1.0000001; i = i + 1.0 / (2.0 * blur_quality + 1.0)) {
        for(var j: f32 = -1.0; j < 1.0000001; j = j + 1.0 / (2.0 * blur_quality + 1.0)) {
            let blur_distance = hypot(i, j);
            let blur_weighting = 1.0 - blur_distance / sqrt(2.0);
            blur_weighting_sum = blur_weighting_sum + blur_weighting;
            blur_brightness_sum = blur_brightness_sum + blur_weighting * hex_base(hex_base_line_thickness * shockwave_factor, hex_uv + vec2<f32>(i * hex_base_blur_size, j * hex_base_blur_size), scale_x, scale_y);
        }
    }

    // Create the hex pattern based on the "Shapes" examples in the Book of Shaders
    let hex_base_bw = blur_brightness_sum / blur_weighting_sum; // Divide the sum by the number of sample points

    // Blurred hex glow
    // Reset basics
    blur_brightness_sum = 0.0;
    blur_weighting_sum = 0.0;
    for(var i: f32 = -1.0; i < 1.0000001; i = i + 1.0 / (2.0 * blur_quality + 1.0)) {
        for(var j: f32 = -1.0; j < 1.0000001; j = j + 1.0 / (2.0 * blur_quality + 1.0)) {
            let blur_distance = hypot(i, j);
            let blur_weighting = 1.0 - blur_distance / sqrt(2.0);
            blur_weighting_sum = blur_weighting_sum + blur_weighting;
            blur_brightness_sum = blur_brightness_sum + blur_weighting * hex_base(hex_glow_line_thickness * shockwave_factor, hex_uv + vec2<f32>(i * hex_glow_blur_size, j * hex_glow_blur_size), scale_x, scale_y);
        }
    }

    // Create the hex pattern based on the "Shapes" examples in the Book of Shaders
    let hex_glow_bw = blur_brightness_sum / blur_weighting_sum; // Divide the sum by the number of sample points

    // Combine BW's
    var combined_bw = 
        0.4 * hex_base_bw
        +
        0.8 * hex_glow_bw
        +
        ring_bw
    ;

//    combined_bw = shockwave_factor;
    combined_bw = combined_bw * ring_activation_factor * ring_deactivation_factor;

    // Colorize the sheild
    let colorf32 = f32(in_color.value); // (0.0 - 3.0 in increments of 1.0)
    let image_colorized = textureSample(texture_gradient, texture_sampler_gradient, vec2<f32>(combined_bw, 0.775 - colorf32 * 0.25)).rgb;

    return vec4<f32>(image_colorized, combined_bw);
}