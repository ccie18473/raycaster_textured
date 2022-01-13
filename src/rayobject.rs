use crate::prelude::*;

pub struct Ray {
    pub buffer: Vec<u8>,
}
impl Ray {
    pub fn new(player: &Player, map: &Map, width: i32, height: i32) -> Ray {
        let mut buffer = vec![0_u8; width as usize * height as usize * 4];

        for x in 0..width {
            //calculate ray position and direction
            let camera_x = (2 * x) as f64 / (width as f64) - 1.0;
            let ray_dir_x = player.dir_x + player.plane_x * camera_x;
            let ray_dir_y = player.dir_y + player.plane_y * camera_x;
            //which box of the map we're in
            let mut map_x = player.pos_x as isize;
            let mut map_y = player.pos_y as isize;
            //length of ray from current position to next x or y-side
            let mut side_dist_x: f64;
            let mut side_dist_y: f64;
            //length of ray from one x or y-side to next x or y-side
            //these are derived as:
            //deltaDistX = sqrt(1 + (rayDirY * rayDirY) / (rayDirX * rayDirX))
            //deltaDistY = sqrt(1 + (rayDirX * rayDirX) / (rayDirY * rayDirY))
            //which can be simplified to abs(|rayDir| / rayDirX) and abs(|rayDir| / rayDirY)
            //where |rayDir| is the length of the vector (rayDirX, rayDirY). Its length,
            //unlike (dirX, dirY) is not 1, however this does not matter, only the
            //ratio between deltaDistX and deltaDistY matters, due to the way the DDA
            //stepping further below works. So the values can be computed as below.
            // Division through zero is prevented, even though technically that's not
            // needed in C++ with IEEE 754 floating point values.
            let delta_dist_x = if ray_dir_x == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir_x).abs()
            };
            let delta_dist_y = if ray_dir_y == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir_y).abs()
            };

            let perp_wall_dist: f64;
            //what direction to step in x or y-direction (either +1 or -1)
            let step_x: isize;
            let step_y: isize;

            let mut hit: usize = 0;
            let mut side: usize = 0;

            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (player.pos_x - map_x as f64) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f64 + 1.0 - player.pos_x) * delta_dist_x;
            }
            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (player.pos_y - map_y as f64) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = (map_y as f64 + 1.0 - player.pos_y) * delta_dist_y;
            }
            //perform DDA
            while hit == 0 {
                //jump to next map square, either in x-direction, or in y-direction
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }
                //Check if ray has hit a wall
                if map.table[map_x as usize][map_y as usize] > 0 {
                    hit = 1;
                }
            }
            //Calculate distance projected on camera direction. This is the shortest distance from the point where the wall is
            //hit to the camera plane. Euclidean to center camera point would give fisheye effect!
            //This can be computed as (mapX - posX + (1 - stepX) / 2) / rayDirX for side == 0, or same formula with Y
            //for size == 1, but can be simplified to the code below thanks to how sideDist and deltaDist are computed:
            //because they were left scaled to |rayDir|. sideDist is the entire length of the ray above after the multiple
            //steps, but we subtract deltaDist once because one step more into the wall was taken above.
            if side == 0 {
                perp_wall_dist = side_dist_x - delta_dist_x;
            } else {
                perp_wall_dist = side_dist_y - delta_dist_y;
            }
            //Calculate height of line to draw on screen
            let pitch: i32 = 100;
            let line_height: i32 = (height as f64 / perp_wall_dist) as i32;
            //calculate lowest and highest pixel to fill in current stripe
            let mut draw_start: i32 = -line_height / 2 + height / 2 + pitch;
            if draw_start < 0 {
                draw_start = 0;
            }
            let mut draw_end: i32 = line_height / 2 + height / 2 + pitch;
            if draw_end >= height {
                draw_end = height - 1;
            }
            //texturing calculations
            let tex_num: u8 = map.table[map_x as usize][map_y as usize] - 1; //1 subtracted from it so that texture 0 can be used!
            //calculate value of wallX
            let mut wall_x: f64; //where exactly the wall was hit
            if side == 0 {
                wall_x = player.pos_y + perp_wall_dist * ray_dir_y;
            } else {
                wall_x = player.pos_x + perp_wall_dist * ray_dir_x;
            }
            wall_x -= wall_x.floor();

            //x coordinate on the texture
            let mut tex_x: i32 = (wall_x * (TEX_WIDTH as f64)) as i32;
            if side == 0 && ray_dir_x > 0.0 {
                tex_x = TEX_WIDTH - tex_x - 1;
            }
            if side == 1 && ray_dir_y < 0.0 {
                tex_x = TEX_WIDTH - tex_x - 1;
            }

            // TODO: an integer-only bresenham or DDA like algorithm could make the texture coordinate stepping faster
            // How much to increase the texture coordinate per screen pixel
            let step: f64 = 1.0 * TEX_HEIGHT as f64 / line_height as f64;

            // Starting texture coordinate
            let mut tex_pos: f64 =
                (draw_start - pitch - height / 2 + line_height / 2) as f64 * step;
            for y in draw_start..draw_end {
                // Cast the texture coordinate to integer
                let mut tex_y: i32 = tex_pos as i32;
                // Protect from overflow
                if tex_x > TEX_WIDTH - 1 {
                    tex_x = TEX_WIDTH - 1;
                } else if tex_x < 0 {
                    tex_x = 0;
                }
                if tex_y > TEX_HEIGHT - 1 {
                    tex_y = TEX_HEIGHT - 1;
                } else if tex_y < 0 {
                    tex_y = 0;
                }
                tex_pos += step;
                

                // get texture
                let texture = &map.texture[tex_num as usize] ;
                // get pixel from selected texture
                let mut pixel_r = texture[((TEX_HEIGHT * tex_y + tex_x) * 4) as usize];
                let mut pixel_g = texture[((TEX_HEIGHT * tex_y + tex_x) * 4 + 1) as usize];
                let mut pixel_b = texture[((TEX_HEIGHT * tex_y + tex_x) * 4 + 2) as usize];
                let pixel_a = texture[((TEX_HEIGHT * tex_y + tex_x) * 4 + 3) as usize];
                //make color darker for y-sides: R, G and B byte each divided through two with a "shift" and an "and"
                if side == 1 {
                    //color = (color >> 1) & 8355711;
                    pixel_r = pixel_r / 2;
                    pixel_g = pixel_g / 2;
                    pixel_b = pixel_b / 2;
                    
                }
                // fill the buffer with pixels
                // each pixel is 4 bytes
                // for each x, fill the vertical line y
                let offset = ((y * width + x) * 4) as usize;

                buffer[offset] = pixel_r;
                buffer[offset + 1] = pixel_g;
                buffer[offset + 2] = pixel_b;
                buffer[offset + 3] = pixel_a;
            }
        }
        //draw the pixels as a buffer
        Ray { buffer }
    }
}
