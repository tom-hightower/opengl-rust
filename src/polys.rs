use crate::render_gl::data;

pub const VERT_RADIUS: f32 = 2.0;
pub const CYLINDER_DIVS: i32 = 15;

pub struct Cylinder {
    bottom: Vec<data::gl_vertex_f32>,
    top: Vec<data::gl_vertex_f32>,
    normals: Vec<data::gl_vertex_f32>,
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
        let mut top: Vec<data::gl_vertex_f32> = Vec::new();
        let mut bottom: Vec<data::gl_vertex_f32> = Vec::new();
        let mut normals: Vec<data::gl_vertex_f32> = Vec::new();
        let top_center = data::gl_vertex_f32 {
            d0: x1,
            d1: y1,
            d2: z1,
        };
        let bottom_center = data::gl_vertex_f32 {
            d0: x2,
            d1: y2,
            d2: z2,
        };
        let mid_vec = data::gl_vertex_f32::sub(&top_center, &bottom_center);
        let mut ref_vec = data::gl_vertex_f32::add(
            &mid_vec,
            &data::gl_vertex_f32 {
                d0: 1.0,
                d1: 1.0,
                d2: 1.0,
            },
        );
        let mut diff = data::gl_vertex_f32::sub(&ref_vec, &mid_vec);
        while diff.mag() == (ref_vec.mag() - mid_vec.mag())
            || data::gl_vertex_f32::dot(&ref_vec, &mid_vec) == 0.
        {
            ref_vec = data::gl_vertex_f32::add(
                &ref_vec,
                &data::gl_vertex_f32 {
                    d0: 0.5,
                    d1: 2.0,
                    d2: 1.0,
                },
            );
            diff = data::gl_vertex_f32::sub(&ref_vec, &mid_vec);
        }
        let perp_vec = data::gl_vertex_f32::cross(&ref_vec, &mid_vec);
        let mut u = perp_vec.copy();
        u.norm();
        u.mult(rad);
        let mut v = data::gl_vertex_f32::cross(&mid_vec, &u);
        v.norm();
        v.mult(rad);
        let theta = ((360 / CYLINDER_DIVS) as f32).to_radians();
        for i in 0..CYLINDER_DIVS {
            let mut temp_u = u.copy();
            temp_u.mult((theta * i as f32).cos());
            let mut temp_v = v.copy();
            temp_v.mult((theta * i as f32).sin());
            bottom.push(data::gl_vertex_f32::add(
                &bottom_center,
                &data::gl_vertex_f32::add(&temp_u, &temp_v),
            ));
            top.push(data::gl_vertex_f32::add(
                &top_center,
                &data::gl_vertex_f32::add(&temp_u, &temp_v),
            ));
            normals.push(data::gl_vertex_f32::sub(
                &bottom[i as usize],
                &bottom_center,
            ));
        }
        Cylinder {
            bottom,
            top,
            normals,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: data::gl_vertex_f32,
    pub clr: data::u2_u10_u10_u10_rev_float,
}

impl Vertex {
    pub fn vertex_attrib_pointers(gl: &gl::Gl) {
        let stride = std::mem::size_of::<Self>();
        let location = 0;
        let offset = 0;
        unsafe {
            data::gl_vertex_f32::vertex_attrib_pointer(gl, stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<data::gl_vertex_f32>();
        unsafe {
            data::u2_u10_u10_u10_rev_float::vertex_attrib_pointer(gl, stride, location, offset);
        }
    }
}

pub struct Polygons {
    vertices: Vec<Vertex>,
    quads: Vec<[*mut Vertex; 4]>,
    cylinders: Vec<Cylinder>,
}

impl Polygons {
    pub fn new_vertex(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, a: f32) {
        self.vertices.push(Vertex {
            pos: data::gl_vertex_f32::new(x, y, z),
            clr: data::u2_u10_u10_u10_rev_float::new(r, g, b, a),
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
