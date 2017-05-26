use euclid::*;

pub struct WorldSpace;
pub struct RGBSpace;
pub type WorldPoint = TypedPoint3D<f32, WorldSpace>;
pub type WorldVec = TypedPoint3D<f32, WorldSpace>;
pub type Colour = TypedPoint3D<u8, RGBSpace>;

pub trait Shape {
    fn intersect(&self, obj : &WorldObject, pos : WorldPoint, dir : WorldVec) -> Option<f32>;
}

pub trait Material {
    fn colour(&self, obj : &WorldObject, at : WorldPoint, incident_ray : WorldVec) -> Colour;
}

pub struct WorldObject {
    pub position : WorldPoint,
    pub shape : Box<Shape>,
    pub material : Box<Material>
}

pub struct FlatMaterial {
    pub colour : Colour
}

impl Material for FlatMaterial {
    fn colour(&self, _ : &WorldObject, _ : WorldPoint, _ : WorldVec) -> Colour {
        self.colour
    }
}

pub struct CheckerboardMaterial {
    pub colour1 : Colour,
    pub colour2 : Colour,
    pub repeat : f32
}

impl Material for CheckerboardMaterial {
    fn colour(&self, _ : &WorldObject, at : WorldPoint, _ : WorldVec) -> Colour {
        if ((at.x/self.repeat).floor() + (at.z/self.repeat).floor()) as i32 % 2 == 0 {
            self.colour1
        }
        else {
            self.colour2
        }
    }
}

pub struct GradientMaterial {
    pub from : Colour,
    pub to : Colour,
    pub from_y : f32,
    pub to_y : f32
}

impl Material for GradientMaterial {
    fn colour(&self, _ : &WorldObject, at : WorldPoint, _ : WorldVec) -> Colour {
        let t = ((at.y - self.from_y)/(self.to_y - self.from_y)).min(1f32).max(0f32);
        (self.from.to_f32() + (self.to.to_f32() - self.from.to_f32()) * t).round().cast::<u8>().unwrap()
    }
}

pub struct Sphere {
    pub radius : f32
}

impl Shape for Sphere {
    fn intersect(&self, obj : &WorldObject, pos : WorldPoint, dir : WorldVec) -> Option<f32> {
        let dir = dir.normalize();
        let model_pos = pos - obj.position;
        //Solve for line-sphere intersection:
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let discriminant = dir.dot(model_pos).powi(2)
            - model_pos.dot(model_pos) + self.radius.powi(2);
        if discriminant < 0f32 {
            return None;
        }
        let discriminant = discriminant.sqrt();

        [-dir.dot(model_pos) + discriminant, -dir.dot(model_pos) - discriminant]
            .iter().filter(|&t| *t > 0f32)
            .fold(None, |acc, &x| match acc {
                Some(a) => Some(f32::min(a,x)),
                None => Some(x)
            })
    }
}

// Plane faces up in +y direction by default
pub struct Plane {
    pub width : f32,
    pub height : f32
}

impl Shape for Plane {
    fn intersect(&self, obj : &WorldObject, pos : WorldPoint, dir : WorldVec) -> Option<f32> {
        let dir = dir.normalize();
        let up = WorldVec::new(0f32, 1f32, 0f32);
        //Solve for line-plane intersection:
        // https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection

        let t = (obj.position - pos).dot(up) / dir.dot(up);
        if t < 0f32 {
            return None;
        }
        let intersection = dir*t + pos;

        if intersection.x <= obj.position.x + self.width/2f32
            && intersection.x >= obj.position.x - self.width/2f32
            && intersection.z <= obj.position.z + self.height/2f32
            && intersection.z >= obj.position.z - self.height/2f32 {
            Some(t)
        }
        else {
            None
        }
    }
}