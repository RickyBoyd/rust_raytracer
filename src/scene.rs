extern crate nalgebra_glm as glm;

use crate::light;
use crate::triangle;

pub struct Scene {
	pub lights: Vec<light::PointLight>,
	pub triangles: Vec<triangle::Triangle>,
}

pub fn load_cornell_box_scene() -> Scene {
	let triangles = load_cornell_box();
	Scene{
		triangles: triangles,
		lights: vec![light::PointLight{
			color: glm::vec3(1.0, 1.0, 1.0),
            position: glm::vec3(-0.3, 0.5, -0.7),
            intensity: 2.0
		}]
	}
}

// Loads the Cornell Box. It is scaled to fill the volume:
// -1 <= x <= +1
// -1 <= y <= +1
// -1 <= z <= +1
fn load_cornell_box() -> Vec<triangle::Triangle> {
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
	triangles.push( triangle::Triangle::new( C, B, A, green ) );
	triangles.push( triangle::Triangle::new( C, D, B, green ) );

	// Left wall
	triangles.push( triangle::Triangle::new( A, E, C, purple ) );
	triangles.push( triangle::Triangle::new( C, E, G, purple ) );

	// Right wall
	triangles.push( triangle::Triangle::new( F, B, D, yellow ) );
	triangles.push( triangle::Triangle::new( H, F, D, yellow ) );

	// Ceiling
	triangles.push( triangle::Triangle::new( E, F, G, cyan ) );
	triangles.push( triangle::Triangle::new( F, H, G, cyan ) );

	// Back wall
	triangles.push( triangle::Triangle::new( G, D, C, white ) );
	triangles.push( triangle::Triangle::new( G, H, D, white ) );

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
	triangles.push( triangle::Triangle::new(E, B, A, red) );
	triangles.push( triangle::Triangle::new(E, F, B, red) );

	// Front
	triangles.push( triangle::Triangle::new(F, D, B, red) );
	triangles.push( triangle::Triangle::new(F, H, D, red) );

	// BACK
	triangles.push( triangle::Triangle::new(H, C, D, red) );
	triangles.push( triangle::Triangle::new(H, G, C, red) );

	// LEFT
	triangles.push( triangle::Triangle::new(G, E, C, red) );
	triangles.push( triangle::Triangle::new(E, A, C, red) );

	// TOP
	triangles.push( triangle::Triangle::new(G, F, E, red) );
	triangles.push( triangle::Triangle::new(G, H, F, red) );

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
	triangles.push( triangle::Triangle::new(E, B, A, blue) );
	triangles.push( triangle::Triangle::new(E, F, B, blue) );

	// Front
	triangles.push( triangle::Triangle::new(F, D, B, blue) );
	triangles.push( triangle::Triangle::new(F, H, D, blue) );

	// BACK
	triangles.push( triangle::Triangle::new(H, C, D, blue) );
	triangles.push( triangle::Triangle::new(H, G, C, blue) );

	// LEFT
	triangles.push( triangle::Triangle::new(G, E, C, blue) );
	triangles.push( triangle::Triangle::new(E, A, C, blue) );

	// TOP
	triangles.push( triangle::Triangle::new(G, F, E, blue) );
	triangles.push( triangle::Triangle::new(G, H, F, blue) );


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