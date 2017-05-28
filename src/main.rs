extern crate image;
extern crate euclid;
extern crate rand;

mod world;

use std::fs::File;
use std::path::Path;
use world::*;

fn get_pixel(x : f32, y : f32, width : u32, height : u32, world : &Vec<WorldObject>) -> Colour {
    let focal_length = 1f32;
    let sensor_size = [1f32, height as f32 / width as f32 * 1f32];
    let camera_pos = WorldPoint::new(-5f32, 0f32, 0f32);
    let camera_dir = WorldVec::new(1f32, 0f32, 0f32).normalize();
    let camera_right = WorldVec::new(0f32, 0f32, 1f32).normalize();
    let camera_up = camera_right.cross(camera_dir);

    let ray = (camera_dir*focal_length + camera_right * (x as f32 * sensor_size[0])
        + camera_up * (-y as f32 * sensor_size[1])).normalize();

    cast_ray(camera_pos, ray, 0, world)
}

fn main() {
    let mut imagebuf = image::ImageBuffer::new(1920, 1080);
    let width = imagebuf.width();
    let height = imagebuf.height();

    let world = vec![
        WorldObject {
            position : WorldPoint::new(-2f32, 10f32, -2f32),
            shape: Box::new(Sphere {
                radius : 0.02f32
            }),
            material : Box::new(FlatMaterial {
                colour: Colour::new(200,200,200),
            }),
            light: true
        },
        WorldObject {
            position : WorldPoint::new(4f32, 0f32, 0f32),
            shape: Box::new(Sphere {
                radius : 1f32
            }),
            material : Box::new(LitMaterial {
                absorb: Box::new(FlatMaterial {
                    colour: Colour::new(232, 104, 80),
                }),
                emit: Box::new(FlatMaterial { colour: Colour::new(10, 10, 10) }),
                shininess: 100f32,
                specular_amount: 1f32,
                reflectivity: 1f32,
                refractivity: 0f32,
                roughness: 1f32
            }),
            light: false
        },
        WorldObject {
            position : WorldPoint::new(5f32, 0f32, 3f32),
            shape: Box::new(Sphere {
                radius : 0.6f32
            }),
            material : Box::new(LitMaterial {
                absorb: Box::new(FlatMaterial {
                    colour: Colour::new(232, 104, 80),
                }),
                emit: Box::new(FlatMaterial { colour: Colour::new(10, 10, 10) }),
                shininess: 100f32,
                specular_amount: 1f32,
                reflectivity: 1f32,
                refractivity: 0f32,
                roughness: 100f32
            }),
            light: false
        },
        WorldObject {
            position : WorldPoint::new(5f32, 0f32, -3f32),
            shape: Box::new(Sphere {
                radius : 0.6f32
            }),
            material : Box::new(LitMaterial {
                absorb: Box::new(FlatMaterial {
                    colour: Colour::new(0, 0, 0),
                }),
                emit: Box::new(FlatMaterial { colour: Colour::new(0, 0, 0) }),
                shininess: 100f32,
                specular_amount: 1f32,
                reflectivity: 0f32,
                refractivity: 1f32,
                roughness: 1f32
            }),
            light: false
        },
        WorldObject {
            position : WorldPoint::new(3f32, -2f32, 0f32),
            shape: Box::new(Plane {
                width: 50f32,
                height: 15f32
            }),
            material : Box::new(LitMaterial {
                absorb: Box::new(CheckerboardMaterial {
                    colour1: Colour::new(230, 10, 10),
                    colour2: Colour::new(0, 0, 0),
                    repeat: 1f32
                }),
                emit: Box::new(CheckerboardMaterial {
                    colour1: Colour::new(20, 0, 0),
                    colour2: Colour::new(0, 0, 0),
                    repeat: 1f32
                }),
                shininess: 10f32,
                specular_amount: 0f32,
                reflectivity: 0f32,
                refractivity: 0f32,
                roughness: 1f32
            }),
            light: false
        },
        WorldObject {
            position : WorldPoint::new(0f32, 0f32, 0f32),
            shape: Box::new(Sphere {
                radius : 100f32
            }),
            material : Box::new(GradientMaterial {
                from: Colour::new(255,255,255),
                from_y: -10f32,
                to: Colour::new(135, 206, 235),
                to_y: 3f32
            }),
            light: false
        },
    ];

    for (x,y,pixel) in imagebuf.enumerate_pixels_mut() {
        let colour = get_pixel(x as f32 / width as f32 - 0.5,
                               y as f32 / height as f32 - 0.5,
                               width, height, &world);
        *pixel = image::Rgb([colour.x, colour.y, colour.z]);
    }

    let ref mut out = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(imagebuf).save(out, image::PNG);
}
