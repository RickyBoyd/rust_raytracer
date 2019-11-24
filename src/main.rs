#![allow(non_snake_case)]

extern crate image;
extern crate piston_window;
extern crate nalgebra_glm as glm;

use piston_window::EventLoop;

mod light;
mod triangle;
mod scene;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const EPSILON: f32 = 0.0000001;

fn main() {
	let scene = scene::load_cornell_box_scene();
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

        let color = get_pixel_color(&camera_pos, &ray_dir, &scene.triangles, &scene.lights);
		
        *pixel = image::Rgba([
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
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

fn get_pixel_color(ray_origin: &glm::Vec3, ray_dir: &glm::Vec3, triangles: &Vec<triangle::Triangle>, lights: &Vec<light::PointLight>) -> glm::Vec3 {
	let mut current_intersection = Intersection{
		position: glm::vec3(0.0, 0.0, 0.0),
		distance: std::f32::MAX,
	};
	let mut intersectionIndex: Option<usize> = None;
	for (i, t) in triangles.iter().enumerate() {
		if let Some(intersection) = ray_intersects_triangle(ray_origin, ray_dir, t) {
			if intersection.distance < current_intersection.distance {
				current_intersection = intersection;
				intersectionIndex = Some(i);
			}
		}
	}

	if let Some(index) = intersectionIndex {
		//triangles[index].color
		shade(&current_intersection, &triangles[index], &lights[0], ray_dir)
	} else {
		glm::vec3(0.0, 0.0, 0.0)
	}
}

struct Intersection {
    position: glm::Vec3,
    distance: f32,
}

fn ray_intersects_triangle(ray_origin: &glm::Vec3, 
                           ray_vector: &glm::Vec3, 
                           triangle: &triangle::Triangle) -> Option<Intersection> {
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

fn shade(intersection: &Intersection, triangle: &triangle::Triangle, light: &light::PointLight, ray_dir: &glm::Vec3) -> glm::Vec3 {
	let shade = glm::dot(&triangle.normal, &(-ray_dir));
	let color = (shade).max(0.0);
	println!("shade {}", shade);
	glm::vec3(color, color, color)
}
