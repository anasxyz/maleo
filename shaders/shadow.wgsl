// shadow shader
// instance based, one quad per shadow
// fragment shader computes sdf distance from rounded rect and applies gaussian falloff

struct Instance {
    @location(0) rect: vec4<f32>,       // x, y, w, h in screen pixels
    @location(1) color: vec4<f32>,      // r, g, b, a
    @location(2) params: vec4<f32>,     // corner_radius, blur, offset_x, offset_y
}

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,         // pixel position relative to shadow rect center
    @location(1) color: vec4<f32>,
    @location(2) half_size: vec2<f32>,  // half width/height of the rect
    @location(3) corner_radius: f32,
    @location(4) blur: f32,
}

struct Screen {
    size: vec2<f32>,
}

@group(0) @binding(0) var<uniform> screen: Screen;

var<private> CORNERS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VertexOut {
    let corner = CORNERS[vi];
    let blur = inst.params.y;
    let offset_x = inst.params.z;
    let offset_y = inst.params.w;

    let expand = blur * 2.0;
    let rx = inst.rect.x + offset_x - expand;
    let ry = inst.rect.y + offset_y - expand;
    let rw = inst.rect.z + expand * 2.0;
    let rh = inst.rect.w + expand * 2.0;

    let px = rx + corner.x * rw;
    let py = ry + corner.y * rh;

    let center_x = inst.rect.x + offset_x + inst.rect.z * 0.5;
    let center_y = inst.rect.y + offset_y + inst.rect.w * 0.5;

    var out: VertexOut;
    out.pos = vec4<f32>(
        px / screen.size.x * 2.0 - 1.0,
        1.0 - py / screen.size.y * 2.0,
        0.0, 1.0
    );
    out.uv = vec2<f32>(px - center_x, py - center_y);
    out.color = inst.color;
    out.half_size = vec2<f32>(inst.rect.z * 0.5, inst.rect.w * 0.5);
    out.corner_radius = inst.params.x;
    out.blur = blur;
    return out;
}

fn rounded_rect_sdf(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius, radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn gaussian(x: f32, sigma: f32) -> f32 {
    return exp(-0.5 * (x / sigma) * (x / sigma));
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let sdf = rounded_rect_sdf(in.uv, in.half_size, in.corner_radius);
    let sigma = max(in.blur * 0.5, 0.0001);
    let alpha = gaussian(sdf, sigma) * in.color.a;
    return vec4<f32>(in.color.rgb, alpha);
}
