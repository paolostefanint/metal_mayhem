#[derive(Debug, Copy, Clone)]
pub struct Body {
    pub body_type: BodyType,
    pub aabb: AABB,
}

impl Body {
    pub fn new(aabb: AABB, body_type: BodyType) -> Body {
        Body { body_type, aabb }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BodyType {
    Static,
    Dynamic,
}

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: (f32, f32),
    pub max: (f32, f32),
}

impl AABB {
    pub fn new(min: (f32, f32), max: (f32, f32)) -> AABB {
        AABB { min, max }
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        return self.min.0 <= other.max.0
            && self.max.0 >= other.min.0
            && self.min.1 <= other.max.1
            && self.max.1 >= other.min.1;
    }

    pub fn center(&self) -> (f32, f32) {
        (
            (self.min.0 + self.max.0) / 2.0,
            (self.min.1 + self.max.1) / 2.0,
        )
    }

    pub fn get_collision_axis(&self, other: &AABB) -> Axis {
        let x_overlap = (self.max.0 - other.min.0)
            .abs()
            .min((self.min.0 - other.max.0).abs());
        let y_overlap = (self.max.1 - other.min.1)
            .abs()
            .min((self.min.1 - other.max.1).abs());

        if x_overlap < y_overlap {
            Axis::X
        } else {
            Axis::Y
        }
    }
}

#[derive(Debug)]
pub enum Axis {
    X,
    Y,
}

pub struct CollisionItem {
    pub pid: Option<u8>,
    pub body: Body,
}
