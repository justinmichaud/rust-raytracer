extern crate image;
extern crate euclid;

mod world;

use std::fs::File;
use std::path::Path;
use world::*;

fn get_pixel(x : f32, y : f32, width : u32, height : u32, world : &Vec<WorldObject>) -> Colour {
    let focal_length = 1f32;
    let sensor_size = [1f32, height as f32 / width as f32 * 1f32];
    let camera_pos = WorldPoint::new(-5f32, 0f32, 0f32);
    let camera_dir = WorldVec::new(1f32, 0f32, 0f32).normalize();
    let camera_up = WorldVec::new(0f32, 1f32, 0f32).normalize();
    let camera_right = camera_dir.cross(camera_up);

    let ray = (camera_dir*focal_length + camera_right * (x as f32 * sensor_size[0])
        + camera_up * (-y as f32 * sensor_size[1])).normalize();

    cast_ray(camera_pos, ray, 0, world, 1f32)
}

fn main() {
    let mut imagebuf = image::ImageBuffer::new(854, 480);
    let width = imagebuf.width();
    let height = imagebuf.height();

    let world = vec![
        WorldObject {
            position : WorldPoint::new(2f32, 0.6f32, -0.6f32),
            shape: Box::new(Sphere {
                radius : 0.2f32
            }),
            material : Box::new(FlatMaterial {
                colour: Colour::new(212, 165, 57),
            })
        },
        WorldObject {
            position : WorldPoint::new(4f32, 0f32, 0f32),
            shape: Box::new(Sphere {
                radius : 1f32
            }),
            material : Box::new(ReflectMaterial {
                base: Box::new(FlatMaterial {
                    colour: Colour::new(232, 104, 80),
                }),
                smoothness: 1f32
            })
        },
        WorldObject {
            position : WorldPoint::new(5f32, 0f32, 3f32),
            shape: Box::new(Sphere {
                radius : 1f32
            }),
            material : Box::new(ReflectMaterial {
                base: Box::new(FlatMaterial {
                    colour: Colour::new(232, 104, 80),
                }),
                smoothness: 1f32
            })
        },
        WorldObject {
            position : WorldPoint::new(3f32, -2f32, 0f32),
            shape: Box::new(Plane {
                width: 50f32,
                height: 15f32
            }),
            material : Box::new(CheckerboardMaterial {
                colour1: Colour::new(200, 0, 0),
                colour2: Colour::new(0, 0, 0),
                repeat: 1f32
            })
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
            })
        },];

    for (x,y,pixel) in imagebuf.enumerate_pixels_mut() {
        let colour = get_pixel(x as f32 / width as f32 - 0.5,
                               y as f32 / height as f32 - 0.5,
                               width, height, &world);
        *pixel = image::Rgb([colour.x, colour.y, colour.z]);
    }

    let ref mut out = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(imagebuf).save(out, image::PNG);
}
