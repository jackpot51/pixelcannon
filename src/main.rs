extern crate orbclient;
extern crate orbimage;

use orbclient::window::EventIter;
use orbimage::Image;

use std::time::Instant;

struct Matrix4f32 {
    pub m: [[f32; 4]; 4]
}

impl Matrix4f32 {
    pub fn new() -> Matrix4f32 {
        Matrix4f32{m: [[0f32; 4]; 4]}
    }

    pub fn init_perspective(mut self, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Matrix4f32 {
        let tan_half_fov = (fov / 2f32).tan();
        let z_range = z_near - z_far;

        self.m[0][0] = 1f32 / (tan_half_fov * aspect_ratio);      self.m[0][1] = 0f32;                      self.m[0][2] = 0f32;                            self.m[0][3] = 0f32;
        self.m[1][0] = 0f32;                                      self.m[1][1] = 1f32 / tan_half_fov;       self.m[1][2] = 0f32;                            self.m[1][3] = 0f32;
        self.m[2][0] = 0f32;                                      self.m[2][1] = 0f32;                      self.m[2][2] = (- z_near -z_far) / z_range;     self.m[2][3] = 2f32 * z_far * z_near / z_range;
        self.m[3][0] = 0f32;                                      self.m[3][1] = 0f32;                      self.m[3][2] = 1f32;                            self.m[3][3] = 0f32;

        self
    }

    pub fn init_translation(mut self, x: f32, y: f32, z: f32) -> Matrix4f32 {
        self.m[0][0] = 1f32;    self.m[0][1] = 0f32;    self.m[0][2] = 0f32;    self.m[0][3] = x;
        self.m[1][0] = 0f32;    self.m[1][1] = 1f32;    self.m[1][2] = 0f32;    self.m[1][3] = y;
        self.m[2][0] = 0f32;    self.m[2][1] = 0f32;    self.m[2][2] = 1f32;    self.m[2][3] = z;
        self.m[3][0] = 0f32;    self.m[3][1] = 0f32;    self.m[3][2] = 0f32;    self.m[3][3] = 1f32;

        self
    }

    pub fn init_rotation(mut self, x: f32, y: f32, z: f32) -> Matrix4f32 {

        let mut rx = Matrix4f32::new();
        let mut ry = Matrix4f32::new();
        let mut rz = Matrix4f32::new();

        rz.m[0][0] = z.cos();   rz.m[0][1] = -z.sin();  rz.m[0][2] = 0f32;      rz.m[0][3] = 0f32;
        rz.m[1][0] = z.sin();   rz.m[1][1] = z.cos();   rz.m[1][2] = 0f32;      rz.m[1][3] = 0f32;
        rz.m[2][0] = 0f32;      rz.m[2][1] = 0f32;      rz.m[2][2] = 1f32;      rz.m[2][3] = 0f32;
        rz.m[3][0] = 0f32;      rz.m[3][1] = 0f32;      rz.m[3][2] = 0f32;      rz.m[3][3] = 1f32;

        rx.m[0][0] = 1f32;      rx.m[0][1] = 0f32;      rx.m[0][2] = 0f32;      rx.m[0][3] = 0f32;
        rx.m[1][0] = 0f32;      rx.m[1][1] = x.cos();   rx.m[1][2] = -x.sin();  rx.m[1][3] = 0f32;
        rx.m[2][0] = 0f32;      rx.m[2][1] = x.sin();   rx.m[2][2] = x.cos();   rx.m[2][3] = 0f32;
        rx.m[3][0] = 0f32;      rx.m[3][1] = 0f32;      rx.m[3][2] = 0f32;      rx.m[3][3] = 1f32;

        ry.m[0][0] = y.cos();   ry.m[0][1] = 0f32;      ry.m[0][2] = -y.sin();  ry.m[0][3] = 0f32;
        ry.m[1][0] = 0f32;      ry.m[1][1] = 1f32;      ry.m[1][2] = 0f32;      ry.m[1][3] = 0f32;
        ry.m[2][0] = y.sin();   ry.m[2][1] = 0f32;      ry.m[2][2] = y.cos();   ry.m[2][3] = 0f32;
        ry.m[3][0] = 0f32;      ry.m[3][1] = 0f32;      ry.m[3][2] = 0f32;      ry.m[3][3] = 1f32;

        self.m = rz.mul(&ry.mul(&rx)).m;

        self
    }

    pub fn init_sreenspace_transform(mut self, half_width: f32, half_height: f32) -> Matrix4f32 {
        self.m[0][0] = half_width;  self.m[0][1] = 0f32;            self.m[0][2] = 0f32;    self.m[0][3] = half_width;
        self.m[1][0] = 0f32;        self.m[1][1] = -half_height;    self.m[1][2] = 0f32;    self.m[1][3] = half_height;
        self.m[2][0] = 0f32;        self.m[2][1] = 0f32;            self.m[2][2] = 1f32;    self.m[2][3] = 0f32;
        self.m[3][0] = 0f32;        self.m[3][1] = 0f32;            self.m[3][2] = 0f32;    self.m[3][3] = 1f32;

        self
    }

    pub fn transform(&self, other: &Vector4f32) -> Vector4f32 {
        Vector4f32 {
            x: self.m[0][0] * other.x + self.m[0][1] * other.y + self.m[0][2] * other.z + self.m[0][3] * other.w,
            y: self.m[1][0] * other.x + self.m[1][1] * other.y + self.m[1][2] * other.z + self.m[1][3] * other.w,
            z: self.m[2][0] * other.x + self.m[2][1] * other.y + self.m[2][2] * other.z + self.m[2][3] * other.w,
            w: self.m[3][0] * other.x + self.m[3][1] * other.y + self.m[3][2] * other.z + self.m[3][3] * other.w }
    }

    pub fn mul(&self, other: &Matrix4f32) -> Matrix4f32 {

        let mut ret = Matrix4f32::new();

        for c_idx in 0..4 {
            for r_idx in 0..4 {
                ret.m[c_idx][r_idx] =
                self.m[c_idx][0] * other.m[0][r_idx] +
                self.m[c_idx][1] * other.m[1][r_idx] +
                self.m[c_idx][2] * other.m[2][r_idx] +
                self.m[c_idx][3] * other.m[3][r_idx];
            }
        }

        ret
    }
}

struct Vector4f32 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

struct Vertex {
    pos: Vector4f32,
    tex_coords: Vector4f32 //TODO(dustin): don't waste space here we only need 2 values
}

impl Vertex {

    // pub fn new(_x: f32, _y: f32, _z: f32) -> Vertex {
    //     Vertex{pos: Vector4f32{x: _x, y: _y, z: _z, w: 1f32}}
    // }

    pub fn new_with_pos_and_texcoords(_pos: Vector4f32, _coords: Vector4f32) -> Vertex {
        Vertex{pos: _pos, tex_coords: _coords}
    }

    pub fn calc_double_area(&self, v1: &Vertex, v2: &Vertex) -> i32 {

        let x1 = (v1.pos.x as i32 - self.pos.x as i32) as i32;
        let y1 = (v1.pos.y as i32 - self.pos.y as i32) as i32;
        let x2 = (v2.pos.x as i32 - self.pos.x as i32) as i32;
        let y2 = (v2.pos.y as i32 - self.pos.y as i32) as i32;

        (x1 * y2 - x2 * y1)
    }

    //TODO(dustin): fix this!
    pub fn transform(&self, transform: &Matrix4f32) -> Vertex {
        Vertex::new_with_pos_and_texcoords(transform.transform(&self.pos), Vector4f32{x: self.tex_coords.x, y: self.tex_coords.y, z: self.tex_coords.z, w: self.tex_coords.w})
    }

    pub fn perspective_divide(&self) -> Vertex {
        Vertex::new_with_pos_and_texcoords(Vector4f32{ x: self.pos.x / self.pos.w, y: self.pos.y / self.pos.w, z: self.pos.z / self.pos.w, w: self.pos.w}, Vector4f32{x: self.tex_coords.x, y: self.tex_coords.y, z: self.tex_coords.z, w: self.tex_coords.w})
    }

}

struct Edge {
    pos_x: f32,
    step_x: f32,
    start_y: i32,
    end_y: i32,

    tex_coords_x: f32,
    tex_coords_step_x: f32,
    tex_coords_y: f32,
    tex_coords_step_y: f32,
    one_over_z: f32,
    one_over_step_z: f32
}

impl Edge {
    pub fn new(interpolator: &Interpolator, min_vert: &Vertex, max_vert: &Vertex, min_y_vert_index: i32) -> Edge {

        let dist_y = max_vert.pos.y - min_vert.pos.y;
        let dist_x = max_vert.pos.x - min_vert.pos.x;
        let prestep_y = min_vert.pos.y.ceil() - min_vert.pos.y;
        let _step_x = dist_x as f32 / dist_y as f32;
        let _pos_x = min_vert.pos.x + prestep_y * _step_x;
        let prestep_x = _pos_x - min_vert.pos.x;

        let _tex_coord_x = interpolator.tex_coords_x[min_y_vert_index as usize] +
            interpolator.tex_coords_step_xx * prestep_x +
            interpolator.tex_coords_step_xy * prestep_y;
        let _tex_coord_step_x = interpolator.tex_coords_step_xy + interpolator.tex_coords_step_xx * _step_x;

        let _tex_coord_y = interpolator.tex_coords_y[min_y_vert_index as usize] +
            interpolator.tex_coords_step_yx * prestep_x +
            interpolator.tex_coords_step_yy * prestep_y;
        let _tex_coord_step_y = interpolator.tex_coords_step_yy + interpolator.tex_coords_step_yx * _step_x;

        let _one_over_z = interpolator.one_over_z[min_y_vert_index as usize] +
            interpolator.one_over_step_zx * prestep_x +
            interpolator.one_over_step_zy * prestep_y;
        let _one_over_step_z = interpolator.one_over_step_zy + interpolator.one_over_step_zx * _step_x;


        Edge {
            pos_x: _pos_x,
            step_x: _step_x,
            start_y: min_vert.pos.y.ceil() as i32,
            end_y: max_vert.pos.y.ceil() as i32,

            tex_coords_x: _tex_coord_x,
            tex_coords_step_x: _tex_coord_step_x,
            tex_coords_y: _tex_coord_y,
            tex_coords_step_y: _tex_coord_step_y,
            one_over_z: _one_over_z,
            one_over_step_z: _one_over_step_z
        }
    }

    pub fn step(&mut self) {
        self.pos_x += self.step_x;
        self.tex_coords_x += self.tex_coords_step_x;
        self.tex_coords_y += self.tex_coords_step_y;
        self.one_over_z += self.one_over_step_z;
    }
}

struct Interpolator {
    tex_coords_x: [f32; 3],
    tex_coords_y: [f32; 3],
    one_over_z: [f32; 3],

    tex_coords_step_xx: f32,
    tex_coords_step_xy: f32,
    tex_coords_step_yx: f32,
    tex_coords_step_yy: f32,

    one_over_step_zx: f32,
    one_over_step_zy: f32
}

impl Interpolator {

    pub fn new(min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex) -> Interpolator {
        let one_over_dx = 1_f32 /
            (((mid_vert.pos.x - max_vert.pos.x) *
            (min_vert.pos.y - max_vert.pos.y)) -
            ((min_vert.pos.x - max_vert.pos.x) *
            (mid_vert.pos.y - max_vert.pos.y)));

        let one_over_dy = -one_over_dx;

        let mut _one_over_z = [0f32; 3];
        let mut _tex_coords_x = [0f32; 3];
        let mut _tex_coords_y = [0f32; 3];

        let mut _tex_coords_step_xx = 0f32;
        let mut _tex_coords_step_xy = 0f32;
        let mut _tex_coords_step_yx = 0f32;
        let mut _tex_coords_step_yy = 0f32;

        let mut _one_over_step_zx = 0f32;
        let mut _one_over_step_zy = 0f32;

        _one_over_z[0] = 1.0f32/min_vert.pos.w;
        _one_over_z[1] = 1.0f32/mid_vert.pos.w;
        _one_over_z[2] = 1.0f32/max_vert.pos.w;

        _tex_coords_x[0] = min_vert.tex_coords.x * _one_over_z[0];
        _tex_coords_x[1] = mid_vert.tex_coords.x * _one_over_z[1];
        _tex_coords_x[2] = max_vert.tex_coords.x * _one_over_z[2];

        _tex_coords_y[0] = min_vert.tex_coords.y * _one_over_z[0];
        _tex_coords_y[1] = mid_vert.tex_coords.y * _one_over_z[1];
        _tex_coords_y[2] = max_vert.tex_coords.y * _one_over_z[2];

        _tex_coords_step_xx = Interpolator::calc_step_x(_tex_coords_x, min_vert, mid_vert, max_vert, one_over_dx);
        _tex_coords_step_xy = Interpolator::calc_step_y(_tex_coords_x, min_vert, mid_vert, max_vert, one_over_dy);
        _tex_coords_step_yx = Interpolator::calc_step_x(_tex_coords_y, min_vert, mid_vert, max_vert, one_over_dx);
        _tex_coords_step_yy = Interpolator::calc_step_y(_tex_coords_y, min_vert, mid_vert, max_vert, one_over_dy);
        _one_over_step_zx = Interpolator::calc_step_x(_one_over_z, min_vert, mid_vert, max_vert, one_over_dx);
        _one_over_step_zy = Interpolator::calc_step_y(_one_over_z, min_vert, mid_vert, max_vert, one_over_dy);

        Interpolator{
            tex_coords_x: _tex_coords_x,
            tex_coords_y: _tex_coords_y,
            one_over_z: _one_over_z,

            tex_coords_step_xx: _tex_coords_step_xx,
            tex_coords_step_xy: _tex_coords_step_xy,
            tex_coords_step_yx: _tex_coords_step_yx,
            tex_coords_step_yy: _tex_coords_step_yy,

            one_over_step_zx: _one_over_step_zx,
            one_over_step_zy: _one_over_step_zy
        }
    }

    fn calc_step_x(values: [f32; 3], min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, one_over_dx: f32) -> f32 {
        let val =   (((values[1] - values[2]) *
                    (min_vert.pos.y - max_vert.pos.y)) -
                    ((values[0] - values[2]) *
                    (mid_vert.pos.y - max_vert.pos.y))) * one_over_dx;

        val
    }

    fn calc_step_y(values: [f32; 3], min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, one_over_dy: f32) -> f32 {
        let val =   (((values[1] - values[2]) *
                    (min_vert.pos.x - max_vert.pos.x)) -
                    ((values[0] - values[2]) *
                    (mid_vert.pos.x - max_vert.pos.x))) * one_over_dy;

        val
    }
}

//NOTE(dustin): format ARGB
struct BitmapTexture {
    width: i32,
    height: i32,
    data: Vec<u8>
}

//TODO:(dustin) use orbclient color format, avoid expensive conversation
impl BitmapTexture {
    pub fn new(_width: i32, _height: i32) -> BitmapTexture {
        BitmapTexture {
            width: _width,
            height: _height,
            data: vec![0_u8; (_width * _height * 4) as usize]
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, a: u8, r: u8, g: u8, b: u8) {

        let idx = ((x + y * self.width) * 4) as usize;
        self.data[idx    ] = a;
        self.data[idx + 1] = r;
        self.data[idx + 2] = g;
        self.data[idx + 3] = b;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> (u8, u8, u8, u8) {
        let tex_idx = ((x + y * self.width) * 4) as usize;

        let a = self.data[tex_idx];
        let r = self.data[tex_idx + 1];
        let g = self.data[tex_idx + 2];
        let b = self.data[tex_idx + 3];

        (a, r, g, b)
    }

    pub fn get_orb_pixel(&self, x: i32, y: i32) -> orbclient::Color {
        let (a, r, g, b) = self.get_pixel(x, y);
        let color = ((a as u32) << 24) + ((r as u32) << 16) + ((g as u32) << 8) + b as u32;

        orbclient::Color { data: color }
    }

    pub fn from_orbimage(image: &Image) -> BitmapTexture {
        let mut texture = BitmapTexture::new(image.width() as i32, image.height() as i32);

        for x in 0..texture.width {
            for y in 0..texture.height {

                let col_idx = (x + y * image.width() as i32) as usize;
                let orb_color = image.data()[col_idx];

                let r = (orb_color.data >> 16) as u8;
                let g = (orb_color.data >> 8) as u8;
                let b = orb_color.data as u8;
                texture.set_pixel(x, y, 255, r, g, b);
            }
        }

        texture
    }
    // pub fn copy_pixel_from_texture(&mut self, dest_x: i32, dest_y: i32, src_x: i32, src_y: i32, texture: &BitmapTexture) {
    //
    //     let dest_idx = ((dest_x + dest_y * self.width) * 4) as usize;
    //     let src_idx = ((src_x + src_y * texture.width) * 4) as usize;
    //
    //     self.data[dest_idx    ] = texture.data[src_idx];
    //     self.data[dest_idx + 1] = texture.data[src_idx + 1];
    //     self.data[dest_idx + 2] = texture.data[src_idx + 2];
    //     self.data[dest_idx + 3] = texture.data[src_idx + 3];
    // }

    // pub fn copy_to_byte_array(& self, &mut)
}

struct RenderContext {
    window: Box<orbclient::Window>,
}

impl RenderContext {
    pub fn new(width: u32, height: u32, title: &str) -> RenderContext {
        let orb_window = orbclient::Window::new_flags(100, 100, width, height, title, true).unwrap();
        RenderContext{window: orb_window}
    }

    pub fn get_height(&self) -> u32 {
        self.window.height()
    }

    pub fn get_width(&self) -> u32 {
        self.window.width()
    }

    pub fn events(&mut self) -> EventIter {
        self.window.events()
    }

    pub fn clear(&mut self) {
        self.window.set(orbclient::Color { data: 0xFF220CE8});
    }

    pub fn sync(&mut self) {
        self.window.sync();
    }

    pub fn draw_triangle(&mut self, v1: &Vertex, v2: &Vertex, v3: &Vertex, texture: &BitmapTexture) {

        //TODO(dustin): optimisation do not calculate/init every time
        let screen_space_transform = Matrix4f32::new().init_sreenspace_transform(self.get_width() as f32 / 2f32, self.get_height() as f32 / 2f32);

        let mut min_vert = v1.transform(&screen_space_transform).perspective_divide();
        let mut mid_vert = v2.transform(&screen_space_transform).perspective_divide();
        let mut max_vert = v3.transform(&screen_space_transform).perspective_divide();

        if max_vert.pos.y < mid_vert.pos.y {
            std::mem::swap(&mut mid_vert, &mut max_vert);
        }

        if mid_vert.pos.y < min_vert.pos.y {
            std::mem::swap(&mut mid_vert, &mut min_vert);
        }

        if max_vert.pos.y < mid_vert.pos.y {
            std::mem::swap(&mut max_vert, &mut mid_vert);
        }

        self.scan_triangle(&min_vert, &mid_vert, &max_vert, min_vert.calc_double_area(&max_vert, &mid_vert) >= 0, texture);
    }

    fn scan_triangle(&mut self,  min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, side: bool, texture: &BitmapTexture) {

        let interpolator = Interpolator::new(min_vert, mid_vert, max_vert);
        let mut top_to_bottom = Edge::new(&interpolator, min_vert, max_vert, 0);
        let mut top_to_middle = Edge::new(&interpolator, min_vert, mid_vert, 0);
        let mut middle_to_bottom = Edge::new(&interpolator, mid_vert, max_vert, 1);

        self.scan_edges(&mut top_to_bottom, &mut top_to_middle, side, texture);
        self.scan_edges(&mut top_to_bottom, &mut middle_to_bottom, side, texture);
    }

    fn scan_edges(&mut self, first: &mut Edge, second: &mut Edge, side: bool, texture: &BitmapTexture) {

        let start_y = second.start_y;
        let end_y = second.end_y;

        let mut left = first;
        let mut right = second;

        if side {
            std::mem::swap(&mut left, &mut right);
        }

        for idx_y in start_y..end_y {
            self.draw_scan_line(&left, &right, idx_y, texture);
            left.step();
            right.step();
        }
    }

    fn draw_scan_line(&mut self, left: &Edge, right: &Edge, idx_y: i32, texture: &BitmapTexture) {

        let min_x = left.pos_x.ceil() as i32;
        let max_x = right.pos_x.ceil()as i32;
        let prestep_x = min_x as f32 - left.pos_x;

        let dist_x = right.pos_x - left.pos_x;
        let tex_coords_step_xx = (right.tex_coords_x - left.tex_coords_x) / dist_x;
        let tex_coords_step_yx = (right.tex_coords_y - left.tex_coords_y) / dist_x;
        let one_over_step_zx = (right.one_over_z - left.one_over_z) / dist_x;

        let mut tex_coords_x = left.tex_coords_x + tex_coords_step_xx * prestep_x;
        let mut tex_coords_y = left.tex_coords_y + tex_coords_step_yx * prestep_x;
        let mut one_over_z = left.one_over_z + one_over_step_zx * prestep_x;

        for idx_x in min_x..max_x {
            let z = 1_f32 / one_over_z;
            let src_x = ((tex_coords_x * z) * (texture.width - 1) as f32 + 0.5_f32) as i32;
            let src_y = ((tex_coords_y * z) * (texture.height - 1) as f32 + 0.5_f32) as i32;

            self.window.pixel(idx_x, idx_y, texture.get_orb_pixel(src_x, src_y));

            one_over_z += one_over_step_zx;
            tex_coords_x += tex_coords_step_xx;
            tex_coords_y += tex_coords_step_yx;
        }
    }
}

fn main() {

    let mut render_context = RenderContext::new(800, 600, "pixelcannon");
    let mut start = Instant::now();

    let min_vert = Vertex::new_with_pos_and_texcoords(Vector4f32{x:-1_f32, y:-1_f32, z:0_f32, w: 1_f32}, Vector4f32{x:0_f32, y:0_f32, z:0_f32, w:0_f32});
    let mid_vert = Vertex::new_with_pos_and_texcoords(Vector4f32{x: 0_f32, y: 1_f32, z:0_f32, w: 1_f32}, Vector4f32{x:0.5_f32, y:1_f32, z:0_f32, w:0_f32});
    let max_vert = Vertex::new_with_pos_and_texcoords(Vector4f32{x: 1_f32, y:-1_f32, z:0_f32, w: 1_f32}, Vector4f32{x:1_f32, y:0_f32, z:0_f32, w:0_f32});

    let projection = Matrix4f32::new().init_perspective(70.0_f32.to_radians(), render_context.get_width() as f32 / render_context.get_height() as f32, 0.1_f32, 1000_f32);

    let image = Image::from_path("assets/img.png").unwrap();
    let texture = BitmapTexture::from_orbimage(&image);

    let mut rot_cnt = 0_f32;

    let mut frame_cnt = 0_f32;
    let mut counter_duration = 0_f32;

    'event: loop {

        {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
            start = Instant::now();
            // println!("{:?} ms", delta_ms);

            // let delta_ms = 100_f32;

            rot_cnt += delta_ms / 1000_f32;
            let translation = Matrix4f32::new().init_translation(0.0_f32, 0.0_f32, 4.3_f32 + rot_cnt.sin() * 2_f32);
            let rotation = Matrix4f32::new().init_rotation(rot_cnt, rot_cnt, 0.0_f32);
            let transform = &projection.mul(&translation.mul(&rotation));

            // let translation = Matrix4f32::new().init_translation(0.0_f32, 0.0_f32, 3_f32);
            // let rotation = Matrix4f32::new().init_rotation(rot_cnt, rot_cnt, rot_cnt);
            // let transform = &projection.mul(&translation.mul(&rotation));

            render_context.clear();
            render_context.draw_triangle(&min_vert.transform(&transform), &mid_vert.transform(&transform), &max_vert.transform(&transform), &texture);
            render_context.sync();

            frame_cnt += 1_f32;
            counter_duration += delta_ms;
            if counter_duration > 1000_f32 {
                println!("FPS: {}", frame_cnt / counter_duration * 1000_f32);
                frame_cnt = 0_f32;
                counter_duration = 0_f32;
            }

            // break 'event;
        }

        for orbital_event in render_context.events() {
            match orbital_event.to_option() {
                orbclient::EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }

    }
}
