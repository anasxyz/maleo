// sdf-based rounded rectangle renderer.
// one instance = one rounded rect.  the vertex shader generates the quad
// procedurally from @builtin(vertex_index) so no position vb is needed.
//
// instance layout (@location must match shape_renderer.rs):
//   0  pos_size     : vec4f  – [x, y, w, h]  screen pixels, top-left origin
//   1  radii        : vec4f  – [corner_radius, border_width, screen_w, screen_h]
//   2  fill_color   : vec4f  – [r, g, b, a]
//   3  border_color : vec4f  – [r, g, b, a]

struct VertexOut {
    @builtin(position) clip_pos : vec4f,

    // pixel-space position of this fragment relative to the rect's center
    @location(0) local_pos    : vec2f,

    // rect half-extents in pixels
    @location(1) half_size    : vec2f,

    @location(2) corner_radius : f32,
    @location(3) border_width  : f32,
    @location(4) fill_color    : vec4f,
    @location(5) border_color  : vec4f,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vi : u32,
    @location(0) pos_size     : vec4f,
    @location(1) radii        : vec4f,
    @location(2) fill_color   : vec4f,
    @location(3) border_color : vec4f,
) -> VertexOut {
    let x = pos_size.x;
    let y = pos_size.y;
    let w = pos_size.z;
    let h = pos_size.w;

    let corner_radius = radii.x;
    let border_width  = radii.y;
    let screen_w      = radii.z;
    let screen_h      = radii.w;

    // 1px bleed on each side so AA smoothstep has room at the edges
    let bleed = 1.0;
    let bx = x - bleed;
    let by = y - bleed;
    let bw = w + bleed * 2.0;
    let bh = h + bleed * 2.0;

    // two-triangle quad from vertex_index 0-5
    // indices: 0=TL 1=TR 2=BL  3=TR 4=BR 5=BL
    var corners = array<vec2f, 6>(
        vec2f(bx,      by),
        vec2f(bx + bw, by),
        vec2f(bx,      by + bh),
        vec2f(bx + bw, by),
        vec2f(bx + bw, by + bh),
        vec2f(bx,      by + bh),
    );

    let px = corners[vi].x;
    let py = corners[vi].y;

    // ndc conversion (top-left origin → ndc)
    let ndcx =  px / screen_w * 2.0 - 1.0;
    let ndcy = -py / screen_h * 2.0 + 1.0;

    // local position relative to rect center, in pixels
    let cx = x + w * 0.5;
    let cy = y + h * 0.5;

    var out: VertexOut;
    out.clip_pos     = vec4f(ndcx, ndcy, 0.0, 1.0);
    out.local_pos    = vec2f(px - cx, py - cy);
    out.half_size    = vec2f(w * 0.5, h * 0.5);
    out.corner_radius = corner_radius;
    out.border_width  = border_width;
    out.fill_color    = fill_color;
    out.border_color  = border_color;
    return out;
}

// exact signed distance to a rounded rectangle centered at the origin.
// negative = inside, positive = outside.
fn sdf_rounded_rect(p: vec2f, half_size: vec2f, radius: f32) -> f32 {
    let q = abs(p) - half_size + radius;
    return length(max(q, vec2f(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    let d = sdf_rounded_rect(in.local_pos, in.half_size, in.corner_radius);

    // outer edge alpha – 1 inside the shape, 0 outside, smooth over 1px
    let outer_alpha = 1.0 - smoothstep(-0.5, 0.5, d);

    // discard fully transparent fragments early
    if outer_alpha <= 0.0 {
        discard;
    }

    var color = in.fill_color;

    // border: blend in border_color in the ring  -border_width < d < 0
    if in.border_width > 0.0 && in.border_color.a > 0.0 {
        // inner edge of the border (where fill transitions to border)
        let inner_alpha = smoothstep(-0.5, 0.5, d + in.border_width);
        // border_t = 1 at the outer edge, 0 deep inside
        let border_t = inner_alpha * outer_alpha;
        color = mix(in.fill_color, in.border_color, border_t);
    }

    return vec4f(color.rgb, color.a * outer_alpha);
}

