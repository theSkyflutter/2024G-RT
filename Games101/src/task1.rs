#![allow(warnings)]

pub use crate::rasterizer1::{Buffer, Primitive, Rasterizer};
pub use crate::shader::FragmentShaderPayload;
pub use crate::texture::Texture;
pub use crate::utils::*;
pub use nalgebra::Vector3;
pub use opencv::core::Vector;
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::imwrite;
pub use opencv::Result;
pub use std::env;

pub fn t1() -> Result<()> {
    println!("选择任务1");
    let mut angle = 0.0;
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![
        Vector3::new(2.0, 0.0, -2.0),
        Vector3::new(0.0, 2.0, -2.0),
        Vector3::new(-2.0, 0.0, -2.0),
    ];
    let ind = vec![Vector3::new(0, 1, 2)];

    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);

    let mut k = 0;
    let mut frame_count = 0;

    let x: f64;
    let y: f64;
    let z: f64;

    text_io::scan!("{} {} {}", x, y, z);
    let axis = Vector3::from_row_slice(&[x, y, z]);

    let mut toggle_r = false;

    while k != 27 {
        r.clear(Buffer::Both);
        if toggle_r {
            r.set_model(get_model_matrix_with_axis(angle, &axis));
        } else {
            r.set_model(get_model_matrix(angle, 1.0));
        }
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw_triangle(pos_id, ind_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);
        imshow("image", &image).unwrap();

        k = wait_key(80).unwrap();
        println!("frame count: {}", frame_count);
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        } else if k == 'r' as i32 {
            toggle_r = !toggle_r;
        }
        frame_count += 1;
    }
    Ok(())
}
