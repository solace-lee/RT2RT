
struct Sphere {
  center: vec3<f32>,
  color: vec3<f32>,
  radius: f32,
}

struct ObjectData {
  sphere: array<Sphere>,
}

struct Ray {
  direction: vec3<f32>,
  origin: vec3<f32>,
}

struct SceneData {
  cameraPos: vec3<f32>,
  cameraForwards: vec3<f32>,
  cameraRight: vec3<f32>,
  cameraUp: vec3<f32>,
  sphereCount: f32,
}

struct RenderState {
  t: f32,
  color: vec3<f32>,
  hit: bool,
}

@group(0) @binding(0) var color_buffer : texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> scene : SceneData;
@group(0) @binding(2) var<storage, read> objects : ObjectData;


@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {

    let screen_size: vec2<i32> = vec2<i32>(textureDimensions(color_buffer));

    let screen_pos: vec2<i32> = vec2<i32>(i32(GlobalInvocationID.x), i32(GlobalInvocationID.y));

    if screen_pos.x >= screen_size.x || screen_pos.y >= screen_size.y {
        return;
    }

    // 归一化
    let horizontal_coefficient: f32 = (f32(screen_pos.x) - f32(screen_size.x) / 2.0) / f32(screen_size.x);
    let vertical_coefficient: f32 = (f32(screen_pos.y) - f32(screen_size.y) / 2.0) / f32(screen_size.x);

    let forwards: vec3<f32> = scene.cameraForwards;
    let right: vec3<f32> = scene.cameraRight;
    let up: vec3<f32> = scene.cameraUp;

    // var mySphere: Sphere;
    // mySphere.center = vec3<f32>(3.0, 0.0, 0.0);
    // mySphere.radius = 1.0;

    var myRay: Ray;
    myRay.direction = normalize(forwards + horizontal_coefficient * right + vertical_coefficient * up);
    myRay.origin = scene.cameraPos;

    var picel_color: vec3<f32> = rayColor(myRay);

    // if hit(myRay, mySphere) {
    //     picel_color = vec3<f32>(0.5, 1.0, 0.75);
    // }

    textureStore(color_buffer, screen_pos, vec4(picel_color, 1.0));
}

fn rayColor(ray: Ray) -> vec3<f32> {
    var color: vec3<f32> = vec3(0.0, 0.0, 0.0);

    var nearestHit: f32 = 9999;
    var hitSomething: bool = false;

    var renderState: RenderState;

    for (var i: u32 = 0; i < u32(scene.sphereCount); i++) {
        var newRenderState: RenderState = hit(ray, objects.sphere[i], 0.001, nearestHit, renderState);
        if newRenderState.hit {
            nearestHit = newRenderState.t;
            renderState = newRenderState;
            hitSomething = true;
        }
    }

    if hitSomething {
        color = renderState.color;
    }
    return color;
}

fn hit(ray: Ray, sphere: Sphere, tMin: f32, tMax: f32, oldRenderState: RenderState) -> RenderState {
    let co: vec3<f32> = ray.origin - sphere.center;
    let a: f32 = dot(ray.direction, ray.direction);
    let b: f32 = 2.0 * dot(ray.direction, co);
    let c: f32 = dot(co, co) - sphere.radius * sphere.radius;
    let discriminant: f32 = b * b - 4.0 * a * c;

    var renderState: RenderState;
    renderState.color = oldRenderState.color;

    if discriminant > 0.0 {

        let t: f32 = (-b - sqrt(discriminant)) / (2 * a);

        if t > tMin && t < tMax {
            renderState.t = t;
            renderState.color = sphere.color;
            renderState.hit = true;
            return renderState;
        }
    }

    renderState.hit = false;
    return renderState;
}