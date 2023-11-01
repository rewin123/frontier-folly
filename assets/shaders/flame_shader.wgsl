#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings globals

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
// MIT License. © Stefan Gustavson, Munrocket
//
fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn fade2(t: vec2f) -> vec2f { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlinNoise2(P: vec2f) -> f32 {
    var Pi: vec4f = floor(P.xyxy) + vec4f(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4f(0., 0., 1., 1.);
    Pi = Pi % vec4f(289.); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4f = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2f = vec2f(gx.x, gy.x);
    var g10: vec2f = vec2f(gx.y, gy.y);
    var g01: vec2f = vec2f(gx.z, gy.z);
    var g11: vec2f = vec2f(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 *
        vec4f(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2f(fx.x, fy.x));
    let n10 = dot(g10, vec2f(fx.y, fy.y));
    let n01 = dot(g01, vec2f(fx.z, fy.z));
    let n11 = dot(g11, vec2f(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2f(n00, n01), vec2f(n10, n11), vec2f(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}

// Function to generate cloud noise
fn generateCloudNoise(uv: vec2<f32>) -> f32 {
    // Scale the UV coordinates to control the noise size
    let scaledUV = uv;
    
    // Generate multiple layers of Perlin noise to create cloud-like patterns
    var noise = 0.0;
    for (var i = 0; i < 5; i = i + 1) {
        let frequency = pow(2.0, f32(i));
        let amplitude = 1.0 / frequency;
        noise += perlinNoise2(scaledUV * frequency) * amplitude;
    }
    
    return noise;
}



@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let duv = mesh.uv.y;
    let tiling = vec2(1.0, 0.05) * 5.0;
    var alpha = generateCloudNoise(mesh.uv * tiling + vec2(0.0, globals.time) * 2.0) + 1.0;
    alpha = alpha / 2.0;
    alpha = alpha + (duv - 0.5) * 4.5 + 0.3;
    alpha = max(0.0, alpha);
    alpha = min(1.0, alpha);
    alpha = pow(alpha, 2.0);
    let k = 100.0;
    var rgb = material.color.xyz;
    rgb.r = rgb.r * duv;
    rgb.g = rgb.g;
    rgb = rgb * k * alpha;

    return vec4(vec3<f32>(rgb), alpha);
}