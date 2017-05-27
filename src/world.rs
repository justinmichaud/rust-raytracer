use euclid::*;

pub struct WorldSpace;
pub struct RGBSpace;
pub type WorldPoint = TypedPoint3D<f32, WorldSpace>;
pub type WorldVec = TypedPoint3D<f32, WorldSpace>;
pub type Colour = TypedPoint3D<u8, RGBSpace>;

pub fn trace(pos : WorldPoint, dir : WorldVec, world : &Vec<WorldObject>) -> Option<(&WorldObject, f32)> {
    let mut result : Option<(&WorldObject, f32)> = None;

    for s in world.iter() {
        match s.intersect(pos, dir) {
            Some(i) => {
                if result.is_none() || i < result.unwrap().1 {
                    result = Some((s, i))
                }
            }
            None => ()
        }
    }

    result
}

pub fn cast_ray(from : WorldPoint, ray : WorldVec, depth : u8,
                world : &Vec<WorldObject>, smootheness : f32) -> Colour {
    match trace(from + ray*0.01f32, ray, world) {
        Some((obj, t)) => obj.material.colour(obj, from + ray*t, ray, from + ray*0.01f32, world, depth),
        None => Colour::new(0,0,0)
    }
}

pub trait Shape {
    fn intersect(&self, obj : &WorldObject,
                 pos : WorldPoint, dir : WorldVec) -> Option<f32>;

    fn normal(&self, obj : &WorldObject, pos : WorldPoint) -> WorldVec;

    fn contains(&self, obj : &WorldObject, pos : WorldPoint) -> bool;
}

pub trait Material {
    fn colour(&self, obj : &WorldObject, at : WorldPoint, incident_from : WorldPoint,
              incident_ray : WorldVec, world : &Vec<WorldObject>, depth : u8) -> Colour;
}

pub struct WorldObject {
    pub position : WorldPoint,
    pub shape : Box<Shape>,
    pub material : Box<Material>
}

impl WorldObject {
    pub fn intersect(&self, pos : WorldPoint, dir : WorldVec) -> Option<f32> {
        self.shape.intersect(self, pos, dir)
    }

    pub fn normal(&self, pos : WorldPoint) -> WorldPoint {
        self.shape.normal(self, pos)
    }

    pub fn contains(&self, pos : WorldPoint) -> bool {
        self.shape.contains(self, pos)
    }
}

pub struct FlatMaterial {
    pub colour : Colour
}

impl Material for FlatMaterial {
    fn colour(&self, _ : &WorldObject, _ : WorldPoint,
              _ : WorldVec, _ : WorldPoint, _ : &Vec<WorldObject>, _ : u8) -> Colour {
        self.colour
    }
}

pub struct CheckerboardMaterial {
    pub colour1 : Colour,
    pub colour2 : Colour,
    pub repeat : f32
}

impl Material for CheckerboardMaterial {
    fn colour(&self, _ : &WorldObject, at : WorldPoint,
              _ : WorldVec, _ : WorldPoint, _ : &Vec<WorldObject>, _ : u8) -> Colour {
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
    fn colour(&self, _ : &WorldObject, at : WorldPoint,
              _ : WorldVec, _  : WorldPoint, _ : &Vec<WorldObject>, _ : u8) -> Colour {
        let t = ((at.y - self.from_y)/(self.to_y - self.from_y)).min(1f32).max(0f32);
        (self.from.to_f32() + (self.to.to_f32() - self.from.to_f32()) * t).round().cast::<u8>().unwrap()
    }
}

pub struct ReflectMaterial {
    pub smoothness: f32,
    pub base : Box<Material>
}

impl Material for ReflectMaterial {
    fn colour(&self, obj : &WorldObject, at : WorldPoint,
              incident : WorldVec, incident_from : WorldPoint, world : &Vec<WorldObject>, depth : u8) -> Colour {
        if depth > 3 {
            return Colour::new(0,0,0);
        }

        let norm = obj.normal(at);
        let ray = (incident-norm*2f32*incident.dot(norm)).normalize();
        let reflect = cast_ray(at, ray, depth+1, world, self.smoothness).to_f32();
        let base = self.base.colour(obj, at, incident, incident_from, world, depth).to_f32();

        Colour::new((reflect.x*base.x/255f32).min(255f32).max(0f32) as u8,
                    (reflect.y*base.y/255f32).min(255f32).max(0f32) as u8,
                    (reflect.z*base.z/255f32).min(255f32).max(0f32) as u8)
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

    fn normal(&self, obj : &WorldObject, pos : WorldPoint) -> WorldVec {
        (pos - obj.position).normalize()
    }

    fn contains(&self, obj : &WorldObject, pos : WorldPoint) -> bool {
        (pos-obj.position).dot(pos-obj.position).abs() <= self.radius
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

    fn normal(&self, _ : &WorldObject, _ : WorldPoint) -> WorldVec {
        WorldVec::new(0f32, 1f32, 0f32)
    }

    fn contains(&self, obj : &WorldObject, pos : WorldPoint) -> bool {
        pos.x <= obj.position.x + self.width/2f32
            && pos.x >= obj.position.x - self.width/2f32
            && pos.z <= obj.position.z + self.height/2f32
            && pos.z >= obj.position.z - self.height/2f32
            && (pos.y - obj.position.y).abs() < 0.01f32
    }
}