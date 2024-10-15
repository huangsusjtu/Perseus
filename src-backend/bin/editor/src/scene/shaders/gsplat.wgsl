// 顶点着色器

struct VertexInput {
    @location(0) position: vec2f,
    @location(1) index: int,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) v_position: vec2f,
    @location(1) v_color: vec4f,
};

struct UniformParams {
//    u_splat_texture: usampler2D,
    u_splat_texture: sampler,
    projection: vec4f,
    view: vec4f,
    focal: vec2f,
    viewport: vec2f,
    htan_fov: vec2f,
    cam_pos: vec3f,
    splat_scale: float,
}

@group(0) @binding(0) var projection: vec4f;
@group(0) @binding(1) var view: vec4f;
@group(0) @binding(2) var cam_pos: vec3f;
@group(0) @binding(3) var focal: vec2f;
@group(0) @binding(4) var viewport: vec2f;
@group(0) @binding(5) var htan_fov: vec2f;
@group(0) @binding(6) var splat_scale: f32;


@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    // 0x3ffu (1023 in decimal) masks the lower 10 bits of index
    uint u = (uint(in.index) & 0x3ffu) << 1;
    uint v = uint(in.index) >> 10;

    uvec3 pos = texelFetch(params.u_splat_texture, ivec2(u, v), 0).rgb;
    vec3 center = uintBitsToFloat(pos); // splat pos in world space
    vec4 cam = params.view * vec4(center, 1.0);
    vec4 pos2d = params.projection * cam;

    float clip = 1.2 * pos2d.w;
    if (pos2d.z < -clip || pos2d.x < -clip || pos2d.x > clip || pos2d.y < -clip || pos2d.y > clip) {
        gl_Position = vec4(0.0, 0.0, 2.0, 1.0);
        return;
    }

    uvec4 cov = texelFetch(u_splat_texture, ivec2(u | 1u, v), 0);
    // cf. Eq.29 of https://www.cs.umd.edu/~zwicker/publications/EWASplatting-TVCG02.pdf
    vec2 u1 = unpackHalf2x16(cov.x); // a, b
    vec2 u2 = unpackHalf2x16(cov.y); // c, d
    vec2 u3 = unpackHalf2x16(cov.z); // e, f
    // eq.24, symmetric matrix, R * S * S^T * R^T
    mat3 Vrk = mat3(
        u1.x, u1.y, u2.x,
        u1.y, u2.y, u3.x,
        u2.x, u3.x, u3.y
    );

    mat3 view3 = mat3(
        view[0].xyz,
        view[1].xyz,
        view[2].xyz
    );

    // splat pos in camera space
    vec3 t = view3 * (center - params.cam_pos);

    // 3D camera space -> 2D screen space
    float txtz = t.x / t.z;
    float tytz = t.y / t.z;

    float limx = 1.3 * params.htan_fov.x;
    float limy = 1.3 * params.htan_fov.y;

    t.x = clamp(txtz, -limx, limx)*t.z;
    t.y = clamp(tytz, -limy, limy)*t.z;

    // Jacobian for the Taylor approximation of the nonlinear camera->ray transformation (eq.29)
    float tz2 = t.z*t.z;
    mat3 J_T = mat3(
        params.focal.x/t.z, 0., -params.focal.x*t.x/tz2,
        0., params.focal.y/t.z , -params.focal.y*t.y/tz2,
        0., 0., 0.
    );


    mat3 T = transpose(view3) * J_T;

    // covariance matrix in ray space
    mat3 cov2d = transpose(T) * Vrk * T;

    float mid = 0.5*(cov2d[0][0] + cov2d[1][1]);
    float radius = length(vec2(0.5*(cov2d[0][0] - cov2d[1][1]), cov2d[0][1]));
    float lambda1 = mid + radius, lambda2 = mid - radius;

    if (lambda2 < 0.0) return;
    vec2 diagonalVector = normalize(vec2(cov2d[0][1], lambda1 - cov2d[0][0]));
    vec2 majorAxis = min(sqrt(2.0*lambda1), 1024.0) * diagonalVector;
    vec2 minorAxis = min(sqrt(2.0*lambda2), 1024.0) * vec2(diagonalVector.y, -diagonalVector.x);

    vColor = clamp(pos2d.z/pos2d.w+1.0, 0.0, 1.0) * vec4(
        (cov.w) & 0xffu, // 0xffu == 255 in decimal, masks the lowest 8 bits (value in [0, 255])
        (cov.w >> 8) & 0xffu,
        (cov.w >> 16) & 0xffu,
        (cov.w >> 24) & 0xffu
    ) / 255.0;

    vec2 vCenter = vec2(pos2d) / pos2d.w;

    vec2 major = (in.position.x*majorAxis) / params.viewport;
    vec2 minor = (in.position.y*minorAxis) / params.viewport;

    var out: VertexOutput;
    out.clip_position = vec4(vCenter + params.splat_scale*(major + minor), 0.0, 1.0);
    out.v_position = in.position;
    out.v_color = vColor;
    return out;
}

// 片元着色器
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    float A = -dot(in.v_position, in.v_position);
    if (A < -4.0) discard;
    float B = exp(A) * in.v_color.a;
    fragColor = vec4(B * in.v_color.rgb, B);
    return fragColor;
}