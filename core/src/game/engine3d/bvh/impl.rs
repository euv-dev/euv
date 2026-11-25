use super::*;

/// Triangle construction and Möller–Trumbore ray intersection for `BvhTriangle`.
impl BvhTriangle {
    /// Creates a world-space `BvhTriangle` from three vertices and its surface `PbrMaterial`.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - First vertex.
    /// - `Vec3` - Second vertex.
    /// - `Vec3` - Third vertex.
    /// - `PbrMaterial` - Surface PBR material.
    ///
    /// # Returns
    ///
    /// - `Self` - Precomputed BVH triangle record.
    #[must_use]
    pub fn from_points(v0: Vec3, v1: Vec3, v2: Vec3, material: PbrMaterial) -> Self {
        let edge1: Vec3 = v1.sub(v0);
        let edge2: Vec3 = v2.sub(v0);
        let normal: Vec3 = edge1.cross(edge2).normalize();
        let centroid: Vec3 = v0.add(v1).add(v2).scale(1.0 / 3.0);
        let mut bounds: Aabb3D = Aabb3D::empty();
        bounds.expand_point(v0);
        bounds.expand_point(v1);
        bounds.expand_point(v2);
        Self {
            v0,
            edge1,
            edge2,
            normal,
            centroid,
            bounds,
            material,
        }
    }

    /// Tests `ray` against this triangle using Möller–Trumbore returning hit distance in `(t_min, t_max)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Triangle reference.
    /// - `&Ray3D` - Query ray.
    /// - `f32` - Minimum valid distance.
    /// - `f32` - Maximum valid distance.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Parametric hit distance if intersected.
    #[must_use]
    pub fn intersect(&self, ray: &Ray3D, t_min: f32, t_max: f32) -> Option<f32> {
        let e1: Vec3 = self.get_edge1();
        let e2: Vec3 = self.get_edge2();
        let h: Vec3 = ray.get_direction().cross(e2);
        let a: f32 = e1.dot(h);
        if a.abs() < 1e-6 {
            return None;
        }
        let f: f32 = 1.0 / a;
        let s: Vec3 = ray.get_origin().sub(self.get_v0());
        let u: f32 = f * s.dot(h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q: Vec3 = s.cross(e1);
        let v: f32 = f * ray.get_direction().dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t: f32 = f * e2.dot(q);
        if t > t_min && t < t_max {
            Some(t)
        } else {
            None
        }
    }
}

/// Top-down BVH construction and iterative stack-based ray traversal for `Bvh3D`.
impl Bvh3D {
    /// Builds a flat BVH over `triangles` partitioning along each node's longest centroid axis.
    ///
    /// # Arguments
    ///
    /// - `Vec<BvhTriangle>` - World-space triangles to index.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed BVH acceleration structure.
    #[must_use]
    pub fn build(triangles: Vec<BvhTriangle>) -> Self {
        let mut bvh: Self = Self {
            nodes: Vec::with_capacity(triangles.len() * 2 + 1),
            triangles,
        };
        if !bvh.get_triangles().is_empty() {
            let total: usize = bvh.get_triangles().len();
            let _: u32 = bvh.build_node(0, total);
        }
        bvh
    }

    /// Recursively partitions `triangles[start..end]` and appends a `BvhNode3D`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable BVH.
    /// - `usize` - Inclusive start index.
    /// - `usize` - Exclusive end index.
    ///
    /// # Returns
    ///
    /// - `u32` - Index of the created node in `self.nodes`.
    fn build_node(&mut self, start: usize, end: usize) -> u32 {
        let count: usize = end - start;
        let mut bounds: Aabb3D = Aabb3D::empty();
        let mut cent_bounds: Aabb3D = Aabb3D::empty();
        for tri in self.get_triangles()[start..end].iter() {
            bounds.expand_box(tri.get_bounds());
            cent_bounds.expand_point(tri.get_centroid());
        }
        let node_idx: u32 = self.get_nodes().len() as u32;
        self.get_mut_nodes().push(BvhNode3D {
            bounds,
            left: 0,
            right: 0,
            first_tri: start as u32,
            tri_count: count as u32,
        });
        if count <= 4 {
            return node_idx;
        }
        let extent: Vec3 = cent_bounds.get_max().sub(cent_bounds.get_min());
        let axis: usize = if extent.get_x() >= extent.get_y() && extent.get_x() >= extent.get_z() {
            0
        } else if extent.get_y() >= extent.get_z() {
            1
        } else {
            2
        };
        let mid: usize = start + count / 2;
        self.get_mut_triangles()[start..end].sort_by(|a: &BvhTriangle, b: &BvhTriangle| {
            let ca: f32 = match axis {
                0 => a.get_centroid().get_x(),
                1 => a.get_centroid().get_y(),
                _ => a.get_centroid().get_z(),
            };
            let cb: f32 = match axis {
                0 => b.get_centroid().get_x(),
                1 => b.get_centroid().get_y(),
                _ => b.get_centroid().get_z(),
            };
            ca.total_cmp(&cb)
        });
        let left_idx: u32 = self.build_node(start, mid);
        let right_idx: u32 = self.build_node(mid, end);
        let node: &mut BvhNode3D = &mut self.get_mut_nodes()[node_idx as usize];
        node.set_left(left_idx);
        node.set_right(right_idx);
        node.set_tri_count(0);
        node_idx
    }

    /// Traces a shadow/occlusion ray returning `true` immediately upon any hit within `(t_min, t_max)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - BVH reference.
    /// - `&Ray3D` - Shadow ray.
    /// - `f32` - Minimum offset (shadow bias).
    /// - `f32` - Maximum trace distance.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if occluded by scene geometry.
    #[must_use]
    pub fn trace_any_shadow(&self, ray: &Ray3D, t_min: f32, t_max: f32) -> bool {
        let nodes: &[BvhNode3D] = self.get_nodes().as_slice();
        if nodes.is_empty() {
            return false;
        }
        let tris: &[BvhTriangle] = self.get_triangles().as_slice();
        let mut stack: [u32; 64] = [0; 64];
        let mut sp: usize = 1;
        while sp > 0 {
            sp -= 1;
            let n: &BvhNode3D = &nodes[stack[sp] as usize];
            if !n.get_bounds().intersects_ray(ray, t_min, t_max) {
                continue;
            }
            let count: usize = n.get_tri_count() as usize;
            if count > 0 {
                let first: usize = n.get_first_tri() as usize;
                for tri in tris[first..(first + count)].iter() {
                    if tri.intersect(ray, t_min, t_max).is_some() {
                        return true;
                    }
                }
            } else if sp + 2 <= 64 {
                stack[sp] = n.get_left();
                stack[sp + 1] = n.get_right();
                sp += 2;
            }
        }
        false
    }

    /// Traces a reflection/GI ray and returns the closest hit surface within `(t_min, t_max)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - BVH reference.
    /// - `&Ray3D` - Reflection or GI ray.
    /// - `f32` - Minimum offset bias.
    /// - `f32` - Maximum trace distance.
    ///
    /// # Returns
    ///
    /// - `Option<RayHit3D>` - Closest hit metadata if any geometry was struck.
    #[must_use]
    pub fn trace_closest(&self, ray: &Ray3D, t_min: f32, t_max: f32) -> Option<RayHit3D> {
        let nodes: &[BvhNode3D] = self.get_nodes().as_slice();
        if nodes.is_empty() {
            return None;
        }
        let tris: &[BvhTriangle] = self.get_triangles().as_slice();
        let mut stack: [u32; 64] = [0; 64];
        let mut sp: usize = 1;
        let mut best_t: f32 = t_max;
        let mut hit: Option<RayHit3D> = None;
        while sp > 0 {
            sp -= 1;
            let n: &BvhNode3D = &nodes[stack[sp] as usize];
            if !n.get_bounds().intersects_ray(ray, t_min, best_t) {
                continue;
            }
            let count: usize = n.get_tri_count() as usize;
            if count > 0 {
                let first: usize = n.get_first_tri() as usize;
                for tri in tris[first..(first + count)].iter() {
                    if let Some(t) = tri.intersect(ray, t_min, best_t) {
                        best_t = t;
                        let pt: Vec3 = ray.get_origin().add(ray.get_direction().scale(t));
                        hit = Some(RayHit3D {
                            distance: t,
                            point: pt,
                            normal: tri.get_normal(),
                            material: tri.get_material(),
                        });
                    }
                }
            } else if sp + 2 <= 64 {
                stack[sp] = n.get_left();
                stack[sp + 1] = n.get_right();
                sp += 2;
            }
        }
        hit
    }
}
