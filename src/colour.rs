//#![crate_type = "lib"]

use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(pixel_colour: Colour) {

    let r = (255.99 * pixel_colour.x) as u8;
    let g = (255.99 * pixel_colour.y) as u8;
    let b = (255.99 * pixel_colour.z) as u8;

    println!("{} {} {}", r, g, b);

}
