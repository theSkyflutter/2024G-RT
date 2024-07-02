use std::collections::HashMap;

use crate::triangle::Triangle;
use nalgebra::{Matrix4, Vector3, Vector4};

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */
    frame_sample: Vec<[Vector3<f64>; 4]>,
    depth_sample: Vec<[f64; 4]>,
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h) as usize, 0.0);
        r.frame_sample
            .resize((w * h) as usize, [Vector3::zeros(); 4]);
        r.depth_sample.resize((w * h) as usize, [0.0; 4]);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                self.frame_buf.fill(Vector3::zeros());
                self.frame_sample.fill([Vector3::zeros(); 4]);
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
                self.depth_sample.fill([f64::MAX; 4]);
            }
            Buffer::Both => {
                self.frame_buf.fill(Vector3::zeros());
                self.frame_sample.fill([Vector3::zeros(); 4]);
                self.depth_buf.fill(f64::MAX);
                self.depth_sample.fill([f64::MAX; 4]);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(
        &mut self,
        pos_buffer: PosBufId,
        ind_buffer: IndBufId,
        col_buffer: ColBufId,
        _typ: Primitive,
    ) {
        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v = vec![
                mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                mvp * to_vec4(buf[i[1]], Some(1.0)),
                mvp * to_vec4(buf[i[2]], Some(1.0)),
            ];

            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            self.rasterize_triangle(&t);
        }
    }

    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        /*  implement your code here  */

        let v = [t.v[0].xyz(), t.v[1].xyz(), t.v[2].xyz()];

        let min_x = v[0].x.min(v[1].x).min(v[2].x).floor() as usize;
        let max_x = v[0].x.max(v[1].x).max(v[2].x).ceil() as usize;
        let min_y = v[0].y.min(v[1].y).min(v[2].y).floor() as usize;
        let max_y = v[0].y.max(v[1].y).max(v[2].y).ceil() as usize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                // No anti-aliasing
                // if inside_triangle(x as f64 + 0.5, y as f64 + 0.5, &v) {
                //     let (alpha, beta, gamma) = compute_barycentric2d(x as f64, y as f64, &v);
                //     let z = (alpha * t.v[0].z / t.v[0].w
                //         + beta * t.v[1].z / t.v[1].w
                //         + gamma * t.v[2].z / t.v[2].w)
                //         / (alpha / t.v[0].w + beta / t.v[1].w + gamma / t.v[2].w);

                //     let index = self.get_index(x, y);
                //     if self.depth_buf[index] > z {
                //         self.depth_buf[index] = z;
                //         self.frame_buf[index] = t.get_color();
                //     }
                // }

                // MSAA
                {
                    let sample_offsets = [[0.75, 0.75], [0.25, 0.75], [0.25, 0.25], [0.75, 0.25]];
                    let mut samples = [false; 4];
                    let mut update_pixel = false;

                    for i in 0..4 {
                        if inside_triangle(
                            x as f64 + sample_offsets[i][0],
                            y as f64 + sample_offsets[i][1],
                            &v,
                        ) {
                            samples[i] = true;
                            update_pixel = true;
                        }
                    }

                    let index = self.get_index(x, y);

                    for i in 0..4 {
                        if samples[i] {
                            let (alpha, beta, gamma) = compute_barycentric2d(
                                x as f64 + sample_offsets[i][0],
                                y as f64 + sample_offsets[i][1],
                                &v,
                            );
                            let z = alpha * t.v[0].z + beta * t.v[1].z + gamma * t.v[2].z;

                            if self.depth_sample[index][i] > z {
                                self.depth_sample[index][i] = z;
                                self.frame_sample[index][i] = t.get_color();
                            }
                        }
                    }

                    if update_pixel {
                        self.frame_buf[index] = {
                            let mut sum = Vector3::zeros();
                            for i in 0..4 {
                                sum += self.frame_sample[index][i];
                            }
                            sum / 4.0
                        };
                    }
                }
            }
        }

        // FXAA-like
        // Requires to uncomment no anti-aliasing
        // {
        //     let offsets: [[i8; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];

        //     for x in min_x..=max_x {
        //         for y in min_y..=max_y {
        //             let index = self.get_index(x, y);
        //             let mut color = self.frame_buf[index];
        //             let mut count_valid: u8 = 1;

        //             for offset in offsets {
        //                 let x_offset = x as isize + offset[0] as isize;
        //                 let y_offset = y as isize + offset[1] as isize;

        //                 if x_offset >= 0 && y_offset >= 0 {
        //                     let offset_index = self.get_index(x_offset as usize, y_offset as usize);
        //                     if (self.frame_buf[index] - self.frame_buf[offset_index])
        //                         .sum()
        //                         .abs()
        //                         > 0.09
        //                     {
        //                         count_valid += 1;
        //                         color += self.frame_buf[offset_index];
        //                     }
        //                 }
        //             }

        //             self.frame_buf[index] = color / count_valid as f64;
        //         }
        //     }
        // }
    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    /*  implement your code here  */

    let (alpha, beta, gamma) = compute_barycentric2d(x, y, v);

    if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
        true
    } else {
        false
    }
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y
            - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y
            - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y
            - v[1].x * v[0].y);
    (c1, c2, c3)
}
