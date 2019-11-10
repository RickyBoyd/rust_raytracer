#![allow(non_snake_case)]

extern crate image;
extern crate piston_window;
extern crate nalgebra_glm as glm;

use piston_window::EventLoop;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const EPSILON: f32 = 0.0000001;

fn main() {
    let scene = load_cornell_box();
    let camera_pos = glm::vec3(0.0, 0.0, -2.0);

    let mut frame_buffer = image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgba([0,0,0,255]));

    let mut window: piston_window::PistonWindow =
    piston_window::WindowSettings::new("Raytracer", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|_e| { panic!("Could not create window!")});

    for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
		let u = (x as f32 + 0.5) / WIDTH as f32 * 2.0 - 1.0 as f32;
		let v = 1.0 - 2.0 * (y as f32 + 0.5) / HEIGHT as f32;

		let ray_dir = glm::normalize::<f32, glm::U3>(&glm::vec3(u, v, 1.0));

        let mut current_intersection = Intersection{
            position: glm::vec3(0.0, 0.0, 0.0),
            distance: std::f32::MAX,
        };
        let mut intersectionIndex: Option<usize> = None;
        for (i, t) in scene.iter().enumerate() {
            if let Some(intersection) = ray_intersects_triangle(camera_pos, ray_dir, t) {
                if intersection.distance < current_intersection.distance {
                    current_intersection = intersection;
                    println!("intersection with {}!", i);
                    intersectionIndex = Some(i);
                }
            }
        }
        //let mut fragment = glm::vec3(x, y, 0.0);
        let color = if let Some(index) = intersectionIndex {
            scene[index].color
        } else {
            glm::vec3(0.0, 0.0, 0.0)
        };

        *pixel = image::Rgba([
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            //y as u8 % 255,
            (color.z * 255.0) as u8,
            255,
        ]);
    }

    let tex = piston_window::Texture::from_image(
        &mut window.create_texture_context(),
        &frame_buffer,
        &piston_window::TextureSettings::new())
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0; 4], g);
            piston_window::image(&tex, c.transform, g)
        });
    }
}

struct Intersection {
    position: glm::Vec3,
    distance: f32,
}

fn ray_intersects_triangle(ray_origin: glm::Vec3, 
                           ray_vector: glm::Vec3, 
                           triangle: &Triangle) -> Option<Intersection> {
    let edge1 = triangle.v1 - triangle.v0;
    let edge2 = triangle.v2 - triangle.v0;
    let h = glm::cross::<f32, glm::U3>(&ray_vector, &edge2);
    let a: f32 = glm::dot(&edge1, &h);
    if a > -EPSILON && a < EPSILON {
        return None;    // This ray is parallel to this triangle.
    }
    let f: f32 = 1.0 / a;
    let s = ray_origin - triangle.v0;
    let u: f32 = f * glm::dot(&s, &h);
    if u < 0.0 || u > 1.0 {
        return None;
    }
    let q = glm::cross::<f32, glm::U3>(&s, &edge1);
    let v: f32 = f * glm::dot(&ray_vector, &q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    // At this stage we can compute t to find out where the intersection point is on the line.
    let t: f32 = f * glm::dot(&edge2, &q);
    if t > EPSILON && t < 1.0 / EPSILON { // ray intersection
        return Some(Intersection{
                position: triangle.v0 + u * edge1 + v * edge2,
                distance: t,
            });
    }
    return None;
}

// Used to describe a triangular surface
#[derive(Debug)]
struct Triangle {
	v0: glm::Vec3,
	v1: glm::Vec3,
	v2: glm::Vec3,
	normal: glm::Vec3,
	color: glm::Vec3,
}

impl Triangle {
    fn new(v0: glm::Vec3, v1: glm::Vec3, v2: glm::Vec3, color: glm::Vec3) -> Triangle {
        let mut t = Triangle{
            v0,
            v1,
            v2,
            color,
            normal: glm::vec3(0.0, 0.0, 0.0),
        };
        t.recompute_normal();
        t
    }	

    fn recompute_normal(&mut self) {
        let e1 = self.v1 - self.v0;
		let e2 = self.v2 - self.v0;
        let cross = glm::cross::<f32, glm::U3>( &e2, &e1 );
		self.normal = glm::normalize( &cross )
    }
}

// Loads the Cornell Box. It is scaled to fill the volume:
// -1 <= x <= +1
// -1 <= y <= +1
// -1 <= z <= +1
fn load_cornell_box() -> Vec<Triangle> {
	// Defines colors:
    use glm::vec3;

	let red = vec3(0.75, 0.15, 0.15);
	let yellow = vec3(0.75, 0.75, 0.15);
	let green = vec3(0.15, 0.75, 0.15);
	let cyan = vec3(0.15, 0.75, 0.75);
	let blue = vec3(0.15, 0.15, 0.75);
	let purple = vec3(0.75, 0.15, 0.75);
	let white = vec3(0.75, 0.75, 0.75);

    let mut triangles = Vec::with_capacity(5*2*3);

	// ---------------------------------------------------------------------------
	// Room

	let L: f32 = 555.0;			// Length of Cornell Box side.

	let A = vec3(L, 0.0, 0.0);
	let B = vec3(0.0, 0.0, 0.0);
	let C = vec3(L, 0.0, L);
	let D = vec3(0.0, 0.0,L);

	let E = vec3(L, L, 0.0);
	let F = vec3(0.0, L, 0.0);
	let G = vec3(L, L, L);
	let H = vec3(0.0, L, L);

	// Floor:
	triangles.push( Triangle::new( C, B, A, green ) );
	triangles.push( Triangle::new( C, D, B, green ) );

	// Left wall
	triangles.push( Triangle::new( A, E, C, purple ) );
	triangles.push( Triangle::new( C, E, G, purple ) );

	// Right wall
	triangles.push( Triangle::new( F, B, D, yellow ) );
	triangles.push( Triangle::new( H, F, D, yellow ) );

	// Ceiling
	triangles.push( Triangle::new( E, F, G, cyan ) );
	triangles.push( Triangle::new( F, H, G, cyan ) );

	// Back wall
	triangles.push( Triangle::new( G, D, C, white ) );
	triangles.push( Triangle::new( G, H, D, white ) );

	// ---------------------------------------------------------------------------
	// Short block

	let A = vec3(290.0, 0.0, 114.0);
	let B = vec3(130.0, 0.0, 65.0);
	let C = vec3(240.0, 0.0, 272.0);
	let D = vec3( 82.0, 0.0, 225.0);

	let E = vec3(290.0, 165.0, 114.0);
	let F = vec3(130.0, 165.0, 65.0);
	let G = vec3(240.0, 165.0, 272.0);
	let H = vec3( 82.0, 165.0, 225.0);

	// Front
	triangles.push( Triangle::new(E, B, A, red) );
	triangles.push( Triangle::new(E, F, B, red) );

	// Front
	triangles.push( Triangle::new(F, D, B, red) );
	triangles.push( Triangle::new(F, H, D, red) );

	// BACK
	triangles.push( Triangle::new(H, C, D, red) );
	triangles.push( Triangle::new(H, G, C, red) );

	// LEFT
	triangles.push( Triangle::new(G, E, C, red) );
	triangles.push( Triangle::new(E, A, C, red) );

	// TOP
	triangles.push( Triangle::new(G, F, E, red) );
	triangles.push( Triangle::new(G, H, F, red) );

	// ---------------------------------------------------------------------------
	// Tall block

	let A = vec3(423.0, 0.0, 247.0);
	let B = vec3(265.0, 0.0, 296.0);
	let C = vec3(472.0, 0.0, 406.0);
	let D = vec3(314.0, 0.0, 456.0);

	let E = vec3(423.0, 330.0, 247.0);
	let F = vec3(265.0, 330.0, 296.0);
	let G = vec3(472.0, 330.0, 406.0);
	let H = vec3(314.0, 330.0, 456.0);

	// Front
	triangles.push( Triangle::new(E, B, A, blue) );
	triangles.push( Triangle::new(E, F, B, blue) );

	// Front
	triangles.push( Triangle::new(F, D, B, blue) );
	triangles.push( Triangle::new(F, H, D, blue) );

	// BACK
	triangles.push( Triangle::new(H, C, D, blue) );
	triangles.push( Triangle::new(H, G, C, blue) );

	// LEFT
	triangles.push( Triangle::new(G, E, C, blue) );
	triangles.push( Triangle::new(E, A, C, blue) );

	// TOP
	triangles.push( Triangle::new(G, F, E, blue) );
	triangles.push( Triangle::new(G, H, F, blue) );


	// ----------------------------------------------
	// Scale to the volume [-1,1]^3

	for triangle in triangles.iter_mut() {
		triangle.v0 *= 2.0 / L;
		triangle.v1 *= 2.0 / L;
		triangle.v2 *= 2.0 / L;

		triangle.v0 -= vec3(1.0, 1.0, 1.0);
		triangle.v1 -= vec3(1.0, 1.0, 1.0);
		triangle.v2 -= vec3(1.0, 1.0, 1.0);

		//triangles[i].v0.x *= -1;
		//triangles[i].v1.x *= -1;
		//triangles[i].v2.x *= -1;

		//triangle.v0.y *= -1.0;
		//triangle.v1.y *= -1.0;
		//triangle.v2.y *= -1.0;

        //println!("", triangle);

		triangle.recompute_normal();
	}
    return triangles;
}