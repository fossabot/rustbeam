//! Module containing the different surfaces that can be rendered.

use crate::math::{Interval, Ray, Vector3};
use std::f64::{INFINITY, NEG_INFINITY};

struct BoundingBox {
    /// The first corner is the corner that has the lowest coordinate values,
    /// and the second, the highest coordinate values.
    corners: (Vector3, Vector3),
}

impl BoundingBox {
    /// The two corners must be in opposite corners of the bounding box.
    fn new<T: Into<Vector3>>(first_corner: T, second_corner: T) -> Self {
        Self {
            corners: (first_corner.into(), second_corner.into()),
        }
    }

    /// Does the ray intersect the bounding box?
    fn intersects(&self, ray: &Ray) -> bool {
        // We intersect the ray and the 3 cardinal direction slabs generated
        // from the bounding box.
        let mut t_interval = if ray.direction.x != 0.0 {
            // Ray intersects the x-direction slab.
            let t0 = (self.corners.0.x - ray.origin.x) / ray.direction.x;
            let t1 = (self.corners.1.x - ray.origin.x) / ray.direction.x;

            Interval::new(t0, t1)
        } else {
            Interval::new(NEG_INFINITY, INFINITY)
        };

        if ray.direction.y != 0.0 {
            // Ray intersects the y-direction slab.
            let t0 = (self.corners.0.y - ray.origin.y) / ray.direction.y;
            let t1 = (self.corners.1.y - ray.origin.y) / ray.direction.y;

            match t_interval.intersection(Interval::new(t0, t1)) {
                None => return false,
                Some(interval) => t_interval = interval,
            }
        }

        if ray.direction.z != 0.0 {
            // Ray intersects the z-direction slab.
            let t0 = (self.corners.0.z - ray.origin.z) / ray.direction.z;
            let t1 = (self.corners.1.z - ray.origin.z) / ray.direction.z;

            match t_interval.intersection(Interval::new(t0, t1)) {
                None => return false,
                Some(interval) => t_interval = interval,
            }
        }

        let endpoints = t_interval.get_endpoints();
        endpoints.0 >= 0.0 || endpoints.1 >= 0.0
    }
}

/// A `Surface` can intersect a `Ray`.
pub trait Surface {
    /// Find the length along a ray to the first intersection between the ray
    /// and the surface (if any). Also returns the normal of the surface in the
    /// intersection.
    fn closest_intersection(&self, ray: &Ray) -> Option<(f64, Vector3)>;
}

pub struct Plane {
    normal_vec: Vector3,
    distance_from_origin: f64,
}

impl Plane {
    pub fn new<T: Into<Vector3>>(normal_vec: T, distance_from_origin: f64) -> Self {
        Self {
            normal_vec: normal_vec.into().normalize(),
            distance_from_origin,
        }
    }
}

impl Surface for Plane {
    fn closest_intersection(&self, ray: &Ray) -> Option<(f64, Vector3)> {
        let ray_direction_dot_normal = ray.direction.dot(self.normal_vec);
        if ray_direction_dot_normal == 0.0 {
            None
        } else {
            // Ray intersects plane.
            let distance_to_intersection = (self.distance_from_origin
                - ray.origin.dot(self.normal_vec))
                / ray_direction_dot_normal;
            if distance_to_intersection > 0.0 {
                Some((distance_to_intersection, self.normal_vec))
            } else {
                None
            }
        }
    }
}

pub struct Sphere {
    pub center_pos: Vector3,
    /// In meters.
    pub radius: f64,
}

impl Sphere {
    /// Make a sphere with center `center_pos` and radius `radius`.
    pub fn new<T: Into<Vector3>>(center_pos: T, radius: f64) -> Self {
        Self {
            center_pos: center_pos.into(),
            radius,
        }
    }

    /// Compute the minimal bounding box of the sphere.
    fn bounding_box(&self) -> BoundingBox {
        let radius_vec = self.radius * Vector3::ones();
        BoundingBox::new(self.center_pos - radius_vec, self.center_pos + radius_vec)
    }
}

impl Surface for Sphere {
    fn closest_intersection(&self, ray: &Ray) -> Option<(f64, Vector3)> {
        if self.bounding_box().intersects(ray) {
            let origin_to_center = self.center_pos - ray.origin;
            let origin_to_center_dot_dir = origin_to_center.dot(ray.direction);
            let discriminant =
                origin_to_center_dot_dir.powi(2) - (origin_to_center.norm2() - self.radius.powi(2));
            if discriminant.is_sign_negative() {
                // Ray doesn't intersect sphere.
                None
            } else {
                // Ray intersects sphere.
                let mut distance_to_intersection = origin_to_center_dot_dir - discriminant.sqrt();
                if distance_to_intersection <= 0.0 {
                    distance_to_intersection = origin_to_center_dot_dir + discriminant.sqrt();
                    if distance_to_intersection <= 0.0 {
                        return None;
                    }
                }
                let normal =
                    (ray.direction * distance_to_intersection - origin_to_center).normalize();
                Some((distance_to_intersection, normal))
            }
        } else {
            // Ray doesn't intersect bounding box.
            None
        }
    }
}
