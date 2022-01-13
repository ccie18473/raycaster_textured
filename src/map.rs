use crate::prelude::*;

use image;

pub const MAP_WIDTH: usize = 24;
pub const MAP_HEIGHT: usize = 24;

pub const TEX_WIDTH: i32 = 64;
pub const TEX_HEIGHT: i32 = 64;

#[derive(Clone)]
pub struct Map {
    // M Rows, N Columns
    //let mut x = [[0.0; N] ; M];
    pub table: [[u8; MAP_WIDTH]; MAP_HEIGHT],
    pub texture: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(_ctx: &mut Context) -> Map {

        let mut rgba;
        let mut texture = Vec::new();

        let image = include_bytes!("../resources/eagle.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/eagle.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/redbrick.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/purplestone.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/greystone.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/bluestone.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/mossy.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/wood.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);

        let image = include_bytes!("../resources/colorstone.png");
        rgba = image::load_from_memory(image).unwrap().to_rgba().into_raw();
        texture.push(rgba);


        Map {
            table: [
                [
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 7, 7, 7, 7, 7, 7, 7, 7,
                ],
                [
                    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
                ],
                [
                    4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
                ],
                [
                    4, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
                ],
                [
                    4, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
                ],
                [
                    4, 0, 4, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 0, 7, 7, 7, 7, 7,
                ],
                [
                    4, 0, 5, 0, 0, 0, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
                ],
                [
                    4, 0, 6, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    4, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 1,
                ],
                [
                    4, 0, 8, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    4, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
                ],
                [
                    4, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 0, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 1,
                ],
                [
                    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                ],
                [
                    8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
                ],
                [
                    6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
                ],
                [
                    4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 6, 0, 6, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3,
                ],
                [
                    4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
                ],
                [
                    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
                ],
                [
                    4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
                ],
                [
                    4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 2,
                ],
                [
                    4, 0, 0, 5, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
                ],
                [
                    4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
                ],
                [
                    4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
                ],
                [
                    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
                ],
            ],
            texture,
        }
    }
}
