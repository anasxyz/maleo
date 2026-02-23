var<private> QUAD: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(0.0, 0.0),
    vec2f(1.0, 0.0),
    vec2f(0.0, 1.0),
    vec2f(1.0, 0.0),
    vec2f(1.0, 1.0),
    vec2f(0.0, 1.0),
);

struct VertexOut {
    @builtin(position) frag_coord : vec4f,
    @location(0) local_pos    : vec2f,
    @location(1) half_size    : vec2f,
    @location(2) radius       : f32,
    @location(3) border_w     : f32,
    @location(4) aa_width     : f32,
    @location(5) fill_color   : vec4f,
    @location(6) border_color : vec4f,
    @location(7) clip         : vec4f,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vi : u32,
    @location(0) pos_size     : vec4f,
    @location(1) params       : vec4f,
    @location(2) fill_color   : vec4f,
    @location(3) border_color : vec4f,
    @location(4) clip         : vec4f,
    @location(5) screen_size  : vec4f,
) -> VertexOut {
    let x  = pos_size.x;  let y  = pos_size.y;
    let w  = pos_size.z;  let h  = pos_size.w;
    let sw = screen_size.x;  let sh = screen_size.y;

    let radius   = params.x;
    let border_w = params.y;
    let aa_width = params.z;

    let b  = aa_width;
    let qx = x - b;  let qy = y - b;
    let qw = w + b * 2.0;  let qh = h + b * 2.0;

    let c  = QUAD[vi];
    let px = qx + c.x * qw;
    let py = qy + c.y * qh;

    let ndcx =  px / sw * 2.0 - 1.0;
    let ndcy = -(py / sh * 2.0 - 1.0);

    let cx = x + w * 0.5;
    let cy = y + h * 0.5;

    var out: VertexOut;
    out.frag_coord   = vec4f(ndcx, ndcy, 0.0, 1.0);
    out.local_pos    = vec2f(px - cx, py - cy);
    out.half_size    = vec2f(w * 0.5, h * 0.5);
    out.radius       = radius;
    out.border_w     = border_w;
    out.aa_width     = aa_width;
    out.fill_color   = fill_color;
    out.border_color = border_color;
    out.clip         = clip;
    return out;
}

fn sdf_rrect(p: vec2f, half_size: vec2f, radius: f32) -> f32 {
    let q = abs(p) - half_size + radius;
    return length(max(q, vec2f(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn aa_coverage(d: f32) -> f32 {
    let fw = fwidth(d);
    return clamp(0.5 - d / max(fw, 0.0001), 0.0, 1.0);
}

// fs_main takes the full VertexOut struct — @builtin(position) is read from
// in.frag_coord. Do NOT also declare it as a separate parameter, which is
// what caused "Built-in Position present more than once".
@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {

    // clip
    let cl = in.clip;
    if cl.x != 0.0 || cl.y != 0.0 || cl.z != 0.0 || cl.w != 0.0 {
        if in.frag_coord.x < cl.x || in.frag_coord.y < cl.y ||
           in.frag_coord.x > cl.z || in.frag_coord.y > cl.w {
            discard;
        }
    }

    // SDF
    let d = sdf_rrect(in.local_pos, in.half_size, in.radius);

    let outer = aa_coverage(d);
    if outer <= 0.0 { discard; }

    // fill vs border
    var color: vec4f;
    if in.border_w > 0.0 && in.border_color.a > 0.0 {
        let inner = aa_coverage(d + in.border_w);
        color = mix(in.border_color, in.fill_color, inner);
    } else {
        color = in.fill_color;
    }

    return vec4f(color.rgb, color.a * outer);
}
