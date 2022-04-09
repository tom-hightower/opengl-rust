pub const VERT_RADIUS: f32 = 2.0;
pub const CYLINDER_DIVS: i32 = 15;

pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn sub(v1: &Point, v2: &Point) -> Point {
        let x: f32 = v1.x - v2.x;
        let y: f32 = v1.y - v2.y;
        let z: f32 = v1.z - v2.z;
        Point { x, y, z }
    }

    pub fn add(v1: &Point, v2: &Point) -> Point {
        let x: f32 = v1.x + v2.x;
        let y: f32 = v1.y + v2.y;
        let z: f32 = v1.z + v2.z;
        Point { x, y, z }
    }

    pub fn dot(v1: &Point, v2: &Point) -> f32 {
        (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
    }

    pub fn cross(v1: &Point, v2: &Point) -> Point {
        let x = (v1.y * v2.z) - (v1.z * v2.y);
        let y = (v1.z * v2.x) - (v1.x * v2.z);
        let z = (v1.x * v2.y) - (v1.y * v2.x);
        Point { x, y, z }
    }

    pub fn copy(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn mag(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    pub fn norm(&mut self) {
        let mag = self.mag();
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
    }

    pub fn mult(&mut self, factor: f32) {
        self.x = self.x * factor;
        self.y = self.y * factor;
        self.z = self.z * factor;
    }
}

pub struct Cylinder {
    bottom: Vec<Point>,
    top: Vec<Point>,
    normals: Vec<Point>,
}

impl Cylinder {
    pub fn create_cylinder(
        rad: f32,
        x1: f32,
        y1: f32,
        z1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
    ) -> Cylinder {
        let mut top: Vec<Point> = Vec::new();
        let mut bottom: Vec<Point> = Vec::new();
        let mut normals: Vec<Point> = Vec::new();
        let top_center = Point {
            x: x1,
            y: y1,
            z: z1,
        };
        let bottom_center = Point {
            x: x2,
            y: y2,
            z: z2,
        };
        let mid_vec = Point::sub(&top_center, &bottom_center);
        let mut ref_vec = Point::add(
            &mid_vec,
            &Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        );
        let mut diff = Point::sub(&ref_vec, &mid_vec);
        while diff.mag() == (ref_vec.mag() - mid_vec.mag()) || Point::dot(&ref_vec, &mid_vec) == 0.
        {
            ref_vec = Point::add(
                &ref_vec,
                &Point {
                    x: 0.5,
                    y: 2.0,
                    z: 1.0,
                },
            );
            diff = Point::sub(&ref_vec, &mid_vec);
        }
        let perp_vec = Point::cross(&ref_vec, &mid_vec);
        let mut u = perp_vec.copy();
        u.norm();
        u.mult(rad);
        let mut v = Point::cross(&mid_vec, &u);
        v.norm();
        v.mult(rad);
        let theta = ((360 / CYLINDER_DIVS) as f32).to_radians();
        for i in 0..CYLINDER_DIVS {
            let mut temp_u = u.copy();
            temp_u.mult((theta * i as f32).cos());
            let mut temp_v = v.copy();
            temp_v.mult((theta * i as f32).sin());
            bottom.push(Point::add(&bottom_center, &Point::add(&temp_u, &temp_v)));
            top.push(Point::add(&top_center, &Point::add(&temp_u, &temp_v)));
            normals.push(Point::sub(&bottom[i as usize], &bottom_center));
        }
        Cylinder {
            bottom,
            top,
            normals,
        }
    }
}

pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    nx: f32,
    ny: f32,
    nz: f32,
}

pub struct Polygons {
    vertices: Vec<Vertex>,
    quads: Vec<[*mut Vertex; 4]>,
    cylinders: Vec<Cylinder>,
}

impl Polygons {
    pub fn new_vertex(&mut self, x: f32, y: f32, z: f32, nx: f32, ny: f32, nz: f32) {
        self.vertices.push(Vertex {
            x,
            y,
            z,
            nx,
            ny,
            nz,
        });
    }

    pub fn new_quad(
        &mut self,
        vert1: *mut Vertex,
        vert2: *mut Vertex,
        vert3: *mut Vertex,
        vert4: *mut Vertex,
    ) {
        self.quads.push([vert1, vert2, vert3, vert4]);
    }
}
