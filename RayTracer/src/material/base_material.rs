use super::Material;

pub struct BaseMaterial;

impl BaseMaterial {
    pub fn new() -> Self {
        Self
    }
}

impl Material for BaseMaterial {}
