use std::rc::Rc;

use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use super::vec3::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let a = ray.dir.len_sqrd();
        let half_b = dot(&oc, &ray.dir);
        let c = oc.len_sqrd() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.mat = Some(self.mat.clone());
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
