// Used to describe a triangular surface
#[derive(Debug)]
pub struct Triangle {
	pub v0: glm::Vec3,
	pub v1: glm::Vec3,
	pub v2: glm::Vec3,
	pub normal: glm::Vec3,
	pub color: glm::Vec3,
}

impl Triangle {
    pub fn new(v0: glm::Vec3, v1: glm::Vec3, v2: glm::Vec3, color: glm::Vec3) -> Triangle {
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

    pub fn recompute_normal(&mut self) {
        let e1 = self.v1 - self.v0;
		let e2 = self.v2 - self.v0;
        let cross = glm::cross::<f32, glm::U3>( &e2, &e1 );
		self.normal = glm::normalize( &cross )
    }
}