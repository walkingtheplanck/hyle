// DDA ray-voxel compute shader for Hyle viewer.

struct Camera {
    eye: vec3<f32>,
    _pad0: f32,
    forward: vec3<f32>,
    _pad1: f32,
    right: vec3<f32>,
    _pad2: f32,
    up: vec3<f32>,
    _pad3: f32,
    half_w: f32,
    half_h: f32,
    inv_w: f32,
    inv_h: f32,
    aabb_min: vec3<f32>,
    _pad4: f32,
    aabb_max: vec3<f32>,
    max_steps: f32,
    voxel_scale: f32,
    _scale_pad0: f32,
    _scale_pad1: f32,
    _scale_pad2: f32,
    // Brush preview
    brush_center: vec3<f32>,
    brush_lo: f32,     // min offset
    brush_hi: f32,     // max offset
    brush_size: f32,   // for sphere radius
    brush_shape: u32,  // 0=cube, 1=sphere
    brush_mode: u32,   // 0=place, 1=delete
    brush_active: u32, // 0=inactive, 1=active
    _brush_pad: u32,
};

@group(0) @binding(0) var<uniform> cam: Camera;
@group(0) @binding(1) var voxels: texture_3d<u32>;
@group(0) @binding(2) var<storage, read> palette: array<vec4<f32>>;
@group(0) @binding(3) var output: texture_storage_2d<rgba8unorm, write>;

const SUN_DIR: vec3<f32> = vec3<f32>(0.48, 0.64, 0.6);
const AMBIENT: f32 = 0.25;
const SKY: vec4<f32> = vec4<f32>(0.53, 0.73, 0.92, 1.0);

const BRUSH_DELETE_COLOR: vec3<f32> = vec3<f32>(0.85, 0.15, 0.1);
const BRUSH_PLACE_COLOR: vec3<f32> = vec3<f32>(0.15, 0.7, 0.2);
const BRUSH_REPLACE_COLOR: vec3<f32> = vec3<f32>(0.2, 0.4, 0.9);

fn in_brush(pos: vec3<i32>) -> bool {
    let cx = i32(cam.brush_center.x);
    let cy = i32(cam.brush_center.y);
    let cz = i32(cam.brush_center.z);
    let dx = pos.x - cx;
    let dy = pos.y - cy;
    let dz = pos.z - cz;
    let lo = i32(cam.brush_lo);
    let hi = i32(cam.brush_hi);
    if cam.brush_shape == 0u {
        // Cube: check if offset is within [lo, hi]
        return dx >= lo && dx <= hi && dy >= lo && dy <= hi && dz >= lo && dz <= hi;
    } else {
        // Sphere: check against half-size radius
        let r = cam.brush_size * 0.5;
        let half = cam.brush_size * 0.5;
        let fx = f32(dx) + 0.5 - half;
        let fy = f32(dy) + 0.5 - half;
        let fz = f32(dz) + 0.5 - half;
        return (fx * fx + fy * fy + fz * fz) <= r * r;
    }
}

fn intersect_aabb(origin: vec3<f32>, inv_dir: vec3<f32>, bmin: vec3<f32>, bmax: vec3<f32>) -> vec2<f32> {
    let t1 = (bmin - origin) * inv_dir;
    let t2 = (bmax - origin) * inv_dir;

    let tmin = min(t1, t2);
    let tmax = max(t1, t2);

    let t_near = max(max(tmin.x, tmin.y), tmin.z);
    let t_far  = min(min(tmax.x, tmax.y), tmax.z);

    return vec2<f32>(t_near, t_far);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let out_size = textureDimensions(output);
    if gid.x >= out_size.x || gid.y >= out_size.y {
        return;
    }

    let px = f32(gid.x);
    let py = f32(gid.y);

    // Compute ray direction
    let ndc_x = (2.0 * (px + 0.5) * cam.inv_w) - 1.0;
    let ndc_y = 1.0 - (2.0 * (py + 0.5) * cam.inv_h);

    let dir = normalize(cam.forward + cam.right * (ndc_x * cam.half_w) + cam.up * (ndc_y * cam.half_h));

    // Compute inverse direction
    let inv_dir = vec3<f32>(
        select(1.0 / dir.x, 1e30, abs(dir.x) < 1e-8),
        select(1.0 / dir.y, 1e30, abs(dir.y) < 1e-8),
        select(1.0 / dir.z, 1e30, abs(dir.z) < 1e-8),
    );

    // AABB intersection
    let tt = intersect_aabb(cam.eye, inv_dir, cam.aabb_min, cam.aabb_max);
    let t_near = tt.x;
    let t_far  = tt.y;

    if t_near > t_far || t_far < 0.0 {
        textureStore(output, vec2<i32>(gid.xy), SKY);
        return;
    }

    let t_enter = max(t_near, 0.0);

    // Start point inside AABB
    let vs = i32(cam.voxel_scale);
    let vsf = cam.voxel_scale;
    var start = cam.eye + dir * (t_enter + 0.001);

    // Snap to scale-aligned grid
    var x = i32(floor(start.x / vsf)) * vs;
    var y = i32(floor(start.y / vsf)) * vs;
    var z = i32(floor(start.z / vsf)) * vs;

    let step_x = select(-vs, vs, dir.x >= 0.0);
    let step_y = select(-vs, vs, dir.y >= 0.0);
    let step_z = select(-vs, vs, dir.z >= 0.0);

    // t_delta: distance along ray to cross one coarse cell (size = scale)
    let t_delta_x = abs(inv_dir.x) * vsf;
    let t_delta_y = abs(inv_dir.y) * vsf;
    let t_delta_z = abs(inv_dir.z) * vsf;

    var t_max_x: f32;
    var t_max_y: f32;
    var t_max_z: f32;

    if abs(dir.x) > 1e-8 {
        let boundary = select(f32(x), f32(x + vs), dir.x > 0.0);
        t_max_x = (boundary - start.x) * inv_dir.x;
    } else {
        t_max_x = 1e30;
    }
    if abs(dir.y) > 1e-8 {
        let boundary = select(f32(y), f32(y + vs), dir.y > 0.0);
        t_max_y = (boundary - start.y) * inv_dir.y;
    } else {
        t_max_y = 1e30;
    }
    if abs(dir.z) > 1e-8 {
        let boundary = select(f32(z), f32(z + vs), dir.z > 0.0);
        t_max_z = (boundary - start.z) * inv_dir.z;
    } else {
        t_max_z = 1e30;
    }

    let imin = vec3<i32>(cam.aabb_min);
    let imax = vec3<i32>(cam.aabb_max);

    var normal = vec3<f32>(0.0, 0.0, 0.0);
    let max_steps = i32(cam.max_steps);

    let norm_x = select(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(-1.0, 0.0, 0.0), step_x > 0);
    let norm_y = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, -1.0, 0.0), step_y > 0);
    let norm_z = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(0.0, 0.0, -1.0), step_z > 0);

    let brush_on = cam.brush_active == 1u;

    for (var i = 0; i < max_steps; i = i + 1) {
        // Bounds check
        if x < imin.x || x >= imax.x || y < imin.y || y >= imax.y || z < imin.z || z >= imax.z {
            break;
        }

        // Sample voxel texture at coarse coords (texture layout: width=X, height=Z, depth=Y)
        let lx = (x - imin.x) / vs;
        let ly = (y - imin.y) / vs;
        let lz = (z - imin.z) / vs;
        let mat_id = textureLoad(voxels, vec3<i32>(lx, lz, ly), 0).r;

        let pos = vec3<i32>(x, y, z);

        if mat_id != 0u {
            // Solid voxel hit — palette stores 2 vec4s per material:
            //   palette[mat_id * 2]     = base_color (RGBA)
            //   palette[mat_id * 2 + 1] = emission (RGB pre-multiplied, intensity in .w)
            let base_color = palette[mat_id * 2u];
            let emission = palette[mat_id * 2u + 1u];

            let n_dot_l = max(dot(normal, SUN_DIR), 0.0);
            let diffuse = AMBIENT + n_dot_l * (1.0 - AMBIENT);
            let shaded = base_color.rgb * diffuse;

            // Add emission (self-illuminated, unaffected by lighting)
            let final_color = min(shaded + emission.rgb, vec3<f32>(1.0, 1.0, 1.0));

            if brush_on && in_brush(pos) {
                // Delete preview: red tint
                if cam.brush_mode == 1u {
                    let blended = final_color * 0.4 + BRUSH_DELETE_COLOR * 0.6;
                    textureStore(output, vec2<i32>(gid.xy), vec4<f32>(blended, 1.0));
                    return;
                }
                // Replace preview: blue tint
                if cam.brush_mode == 2u {
                    let blended = final_color * 0.4 + BRUSH_REPLACE_COLOR * 0.6;
                    textureStore(output, vec2<i32>(gid.xy), vec4<f32>(blended, 1.0));
                    return;
                }
            }

            // Normal hit
            textureStore(output, vec2<i32>(gid.xy), vec4<f32>(final_color, 1.0));
            return;
        } else {
            // Air voxel — check for place preview (mode 0 only)
            if brush_on && cam.brush_mode == 0u && in_brush(pos) {
                let n_dot_l = max(dot(normal, SUN_DIR), 0.0);
                let diffuse = AMBIENT + n_dot_l * (1.0 - AMBIENT);
                let color = BRUSH_PLACE_COLOR * diffuse;
                textureStore(output, vec2<i32>(gid.xy), vec4<f32>(color, 1.0));
                return;
            }
        }

        // DDA step
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x = x + step_x;
                t_max_x = t_max_x + t_delta_x;
                normal = norm_x;
            } else {
                z = z + step_z;
                t_max_z = t_max_z + t_delta_z;
                normal = norm_z;
            }
        } else {
            if t_max_y < t_max_z {
                y = y + step_y;
                t_max_y = t_max_y + t_delta_y;
                normal = norm_y;
            } else {
                z = z + step_z;
                t_max_z = t_max_z + t_delta_z;
                normal = norm_z;
            }
        }
    }

    // Miss after max steps
    textureStore(output, vec2<i32>(gid.xy), SKY);
}
