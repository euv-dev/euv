use super::*;

/// Compact hot-geometry ray intersection kernel for `BvhGeomTri`.
impl BvhGeomTri {
    /// Extracts compact 36-byte hot geometry from a `BvhTriangle`.
    ///
    /// # Arguments
    ///
    /// - `&BvhTriangle` - Source world-space triangle.
    ///
    /// # Returns
    ///
    /// - `Self` - L1-cache-compact triangle geometry.
    #[must_use]
    pub(crate) fn from_triangle(tri: &BvhTriangle) -> Self {
        let v0: Vec3 = tri.get_v0();
        let e1: Vec3 = tri.get_edge1();
        let e2: Vec3 = tri.get_edge2();
        Self {
            v0: [v0.get_x(), v0.get_y(), v0.get_z()],
            e1: [e1.get_x(), e1.get_y(), e1.get_z()],
            e2: [e2.get_x(), e2.get_y(), e2.get_z()],
        }
    }

    /// Deferred-division Möller–Trumbore ray-triangle test avoiding division on rejected triangles.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Compact triangle geometry reference.
    /// - `Vec3` - Ray origin.
    /// - `Vec3` - Unit ray direction.
    /// - `f32` - Minimum valid distance.
    /// - `f32` - Maximum valid distance.
    ///
    /// # Returns
    ///
    /// - `Option<f32>` - Parametric hit distance if intersected.
    #[inline(always)]
    #[must_use]
    pub(crate) fn intersect_raw(
        &self,
        origin: Vec3,
        dir: Vec3,
        t_min: f32,
        t_max: f32,
    ) -> Option<f32> {
        let Self { v0, e1, e2 } = self;
        let dx: f32 = dir.get_x();
        let dy: f32 = dir.get_y();
        let dz: f32 = dir.get_z();
        let hx: f32 = dy * e2[2] - dz * e2[1];
        let hy: f32 = dz * e2[0] - dx * e2[2];
        let hz: f32 = dx * e2[1] - dy * e2[0];
        let det: f32 = e1[0] * hx + e1[1] * hy + e1[2] * hz;
        let sx: f32 = origin.get_x() - v0[0];
        let sy: f32 = origin.get_y() - v0[1];
        let sz: f32 = origin.get_z() - v0[2];
        if det > 1e-6 {
            let u_raw: f32 = sx * hx + sy * hy + sz * hz;
            if u_raw < 0.0 || u_raw > det {
                return None;
            }
            let qx: f32 = sy * e1[2] - sz * e1[1];
            let qy: f32 = sz * e1[0] - sx * e1[2];
            let qz: f32 = sx * e1[1] - sy * e1[0];
            let v_raw: f32 = dx * qx + dy * qy + dz * qz;
            if v_raw < 0.0 || u_raw + v_raw > det {
                return None;
            }
            let t_raw: f32 = e2[0] * qx + e2[1] * qy + e2[2] * qz;
            if t_raw <= t_min * det || t_raw >= t_max * det {
                return None;
            }
            Some(t_raw / det)
        } else if det < -1e-6 {
            let u_raw: f32 = sx * hx + sy * hy + sz * hz;
            if u_raw > 0.0 || u_raw < det {
                return None;
            }
            let qx: f32 = sy * e1[2] - sz * e1[1];
            let qy: f32 = sz * e1[0] - sx * e1[2];
            let qz: f32 = sx * e1[1] - sy * e1[0];
            let v_raw: f32 = dx * qx + dy * qy + dz * qz;
            if v_raw > 0.0 || u_raw + v_raw < det {
                return None;
            }
            let t_raw: f32 = e2[0] * qx + e2[1] * qy + e2[2] * qz;
            if t_raw >= t_min * det || t_raw <= t_max * det {
                return None;
            }
            Some(t_raw / det)
        } else {
            None
        }
    }

    /// Zero-division occlusion-only Möller–Trumbore test for shadow rays within `(t_min, t_max)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Compact triangle geometry reference.
    /// - `Vec3` - Ray origin.
    /// - `Vec3` - Unit ray direction.
    /// - `f32` - Minimum valid distance.
    /// - `f32` - Maximum valid distance.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the ray intersects this triangle within `(t_min, t_max)`.
    #[inline(always)]
    #[must_use]
    pub(crate) fn intersect_any_raw(
        &self,
        origin: Vec3,
        dir: Vec3,
        t_min: f32,
        t_max: f32,
    ) -> bool {
        let Self { v0, e1, e2 } = self;
        let dx: f32 = dir.get_x();
        let dy: f32 = dir.get_y();
        let dz: f32 = dir.get_z();
        let hx: f32 = dy * e2[2] - dz * e2[1];
        let hy: f32 = dz * e2[0] - dx * e2[2];
        let hz: f32 = dx * e2[1] - dy * e2[0];
        let det: f32 = e1[0] * hx + e1[1] * hy + e1[2] * hz;
        let sx: f32 = origin.get_x() - v0[0];
        let sy: f32 = origin.get_y() - v0[1];
        let sz: f32 = origin.get_z() - v0[2];
        if det > 1e-6 {
            let u_raw: f32 = sx * hx + sy * hy + sz * hz;
            if u_raw < 0.0 || u_raw > det {
                return false;
            }
            let qx: f32 = sy * e1[2] - sz * e1[1];
            let qy: f32 = sz * e1[0] - sx * e1[2];
            let qz: f32 = sx * e1[1] - sy * e1[0];
            let v_raw: f32 = dx * qx + dy * qy + dz * qz;
            if v_raw < 0.0 || u_raw + v_raw > det {
                return false;
            }
            let t_raw: f32 = e2[0] * qx + e2[1] * qy + e2[2] * qz;
            t_raw > t_min * det && t_raw < t_max * det
        } else if det < -1e-6 {
            let u_raw: f32 = sx * hx + sy * hy + sz * hz;
            if u_raw > 0.0 || u_raw < det {
                return false;
            }
            let qx: f32 = sy * e1[2] - sz * e1[1];
            let qy: f32 = sz * e1[0] - sx * e1[2];
            let qz: f32 = sx * e1[1] - sy * e1[0];
            let v_raw: f32 = dx * qx + dy * qy + dz * qz;
            if v_raw > 0.0 || u_raw + v_raw < det {
                return false;
            }
            let t_raw: f32 = e2[0] * qx + e2[1] * qy + e2[2] * qz;
            t_raw < t_min * det && t_raw > t_max * det
        } else {
            false
        }
    }
}

/// 4-wide Structure-of-Arrays SIMD-friendly Möller–Trumbore kernels for `BvhTri4`.
impl BvhTri4 {
    /// Packs up to 4 contiguous `BvhGeomTri` entries starting at `base_tri_idx` into one `BvhTri4` packet.
    ///
    /// # Arguments
    ///
    /// - `&[BvhGeomTri]` - Slice of 1..=4 source triangles.
    /// - `u32` - Global triangle index of `slice[0]` inside `Bvh3D::triangles`.
    ///
    /// # Returns
    ///
    /// - `Self` - Packed 4-wide SoA packet with zero-edge sentinel padding for trailing lanes.
    #[must_use]
    pub(crate) fn from_geom_chunk(slice: &[BvhGeomTri], base_tri_idx: u32) -> Self {
        let mut v0x: [f32; 4] = [0.0; 4];
        let mut v0y: [f32; 4] = [0.0; 4];
        let mut v0z: [f32; 4] = [0.0; 4];
        let mut e1x: [f32; 4] = [0.0; 4];
        let mut e1y: [f32; 4] = [0.0; 4];
        let mut e1z: [f32; 4] = [0.0; 4];
        let mut e2x: [f32; 4] = [0.0; 4];
        let mut e2y: [f32; 4] = [0.0; 4];
        let mut e2z: [f32; 4] = [0.0; 4];
        let mut tri_indices: [u32; 4] = [u32::MAX; 4];
        for (lane, tri) in slice.iter().take(4).enumerate() {
            v0x[lane] = tri.v0[0];
            v0y[lane] = tri.v0[1];
            v0z[lane] = tri.v0[2];
            e1x[lane] = tri.e1[0];
            e1y[lane] = tri.e1[1];
            e1z[lane] = tri.e1[2];
            e2x[lane] = tri.e2[0];
            e2y[lane] = tri.e2[1];
            e2z[lane] = tri.e2[2];
            tri_indices[lane] = base_tri_idx.wrapping_add(lane as u32);
        }
        Self {
            v0x,
            v0y,
            v0z,
            e1x,
            e1y,
            e1z,
            e2x,
            e2y,
            e2z,
            tri_indices,
        }
    }

    /// Tests all 4 triangles in parallel for any shadow/occlusion hit within `(t_min, t_max)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - 4-wide SoA triangle packet.
    /// - `[f32; 3]` - Ray origin `(ox, oy, oz)`.
    /// - `[f32; 3]` - Unit ray direction `(dx, dy, dz)`.
    /// - `f32` - Minimum valid distance.
    /// - `f32` - Maximum valid distance.
    ///
    /// # Returns
    ///
    /// - `u32` - Global triangle index of the first occluding lane, or `u32::MAX` on miss.
    #[inline(always)]
    #[must_use]
    pub(crate) fn intersect_any_4(
        &self,
        orig: [f32; 3],
        dir: [f32; 3],
        t_min: f32,
        t_max: f32,
    ) -> u32 {
        let Self {
            v0x,
            v0y,
            v0z,
            e1x,
            e1y,
            e1z,
            e2x,
            e2y,
            e2z,
            tri_indices,
        } = self;
        let ox: f32 = orig[0];
        let oy: f32 = orig[1];
        let oz: f32 = orig[2];
        let dx: f32 = dir[0];
        let dy: f32 = dir[1];
        let dz: f32 = dir[2];
        let mut hits: [bool; 4] = [false; 4];
        for i in 0..4 {
            let hx: f32 = dy.mul_add(e2z[i], -(dz * e2y[i]));
            let hy: f32 = dz.mul_add(e2x[i], -(dx * e2z[i]));
            let hz: f32 = dx.mul_add(e2y[i], -(dy * e2x[i]));
            let det: f32 = e1x[i].mul_add(hx, e1y[i].mul_add(hy, e1z[i] * hz));
            let sign: f32 = 1.0_f32.copysign(det);
            let ad: f32 = det.abs();
            let sx: f32 = ox - v0x[i];
            let sy: f32 = oy - v0y[i];
            let sz: f32 = oz - v0z[i];
            let su: f32 = sx.mul_add(hx, sy.mul_add(hy, sz * hz)) * sign;
            let qx: f32 = sy.mul_add(e1z[i], -(sz * e1y[i]));
            let qy: f32 = sz.mul_add(e1x[i], -(sx * e1z[i]));
            let qz: f32 = sx.mul_add(e1y[i], -(sy * e1x[i]));
            let sv: f32 = dx.mul_add(qx, dy.mul_add(qy, dz * qz)) * sign;
            let st: f32 = e2x[i].mul_add(qx, e2y[i].mul_add(qy, e2z[i] * qz)) * sign;
            let uv_ok: bool = su >= 0.0 && sv >= 0.0 && (su + sv) <= ad;
            let t_ok: bool = st > t_min * ad && st < t_max * ad;
            hits[i] = ad > 1e-6 && uv_ok && t_ok;
        }
        if hits[0] {
            tri_indices[0]
        } else if hits[1] {
            tri_indices[1]
        } else if hits[2] {
            tri_indices[2]
        } else if hits[3] {
            tri_indices[3]
        } else {
            u32::MAX
        }
    }

    /// Tests all 4 triangles in parallel and updates `(best_t, best_idx)` on any closer hit within `(t_min, *best_t)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - 4-wide SoA triangle packet.
    /// - `[f32; 3]` - Ray origin `(ox, oy, oz)`.
    /// - `[f32; 3]` - Unit ray direction `(dx, dy, dz)`.
    /// - `f32` - Minimum valid distance.
    /// - `&mut f32` - Closest hit distance tightened on hit.
    /// - `&mut usize` - Closest hit triangle index updated on hit.
    #[inline(always)]
    pub(crate) fn intersect_closest_4(
        &self,
        orig: [f32; 3],
        dir: [f32; 3],
        t_min: f32,
        best_t: &mut f32,
        best_idx: &mut usize,
    ) {
        let Self {
            v0x,
            v0y,
            v0z,
            e1x,
            e1y,
            e1z,
            e2x,
            e2y,
            e2z,
            tri_indices,
        } = self;
        let ox: f32 = orig[0];
        let oy: f32 = orig[1];
        let oz: f32 = orig[2];
        let dx: f32 = dir[0];
        let dy: f32 = dir[1];
        let dz: f32 = dir[2];
        let cur_max: f32 = *best_t;
        let mut ad_arr: [f32; 4] = [0.0; 4];
        let mut st_arr: [f32; 4] = [0.0; 4];
        let mut hits: [bool; 4] = [false; 4];
        for i in 0..4 {
            let hx: f32 = dy.mul_add(e2z[i], -(dz * e2y[i]));
            let hy: f32 = dz.mul_add(e2x[i], -(dx * e2z[i]));
            let hz: f32 = dx.mul_add(e2y[i], -(dy * e2x[i]));
            let det: f32 = e1x[i].mul_add(hx, e1y[i].mul_add(hy, e1z[i] * hz));
            let sign: f32 = 1.0_f32.copysign(det);
            let ad: f32 = det.abs();
            let sx: f32 = ox - v0x[i];
            let sy: f32 = oy - v0y[i];
            let sz: f32 = oz - v0z[i];
            let su: f32 = sx.mul_add(hx, sy.mul_add(hy, sz * hz)) * sign;
            let qx: f32 = sy.mul_add(e1z[i], -(sz * e1y[i]));
            let qy: f32 = sz.mul_add(e1x[i], -(sx * e1z[i]));
            let qz: f32 = sx.mul_add(e1y[i], -(sy * e1x[i]));
            let sv: f32 = dx.mul_add(qx, dy.mul_add(qy, dz * qz)) * sign;
            let st: f32 = e2x[i].mul_add(qx, e2y[i].mul_add(qy, e2z[i] * qz)) * sign;
            ad_arr[i] = ad;
            st_arr[i] = st;
            let uv_ok: bool = su >= 0.0 && sv >= 0.0 && (su + sv) <= ad;
            let t_ok: bool = st > t_min * ad && st < cur_max * ad;
            hits[i] = ad > 1e-6 && uv_ok && t_ok;
        }
        let any_01: bool = hits[0] || hits[1];
        let any_23: bool = hits[2] || hits[3];
        if any_01 || any_23 {
            for i in 0..4 {
                if hits[i] {
                    let t: f32 = st_arr[i] / ad_arr[i];
                    if t < *best_t {
                        *best_t = t;
                        *best_idx = tri_indices[i] as usize;
                    }
                }
            }
        }
    }
}

/// Scalar-register FMA ray-box slab kernel for 32-byte `BvhFlatNode`.
impl BvhFlatNode {
    /// Computes branchless FMA near-entry slab intersection distance, or `f32::INFINITY` on miss.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Flat node reference.
    /// - `[f32; 3]` - Precomputed safe reciprocal direction `(ix, iy, iz)`.
    /// - `[f32; 3]` - Precomputed negated origin-times-inverse `(-ox * ix, -oy * iy, -oz * iz)`.
    /// - `f32` - Minimum valid distance.
    /// - `f32` - Maximum valid distance.
    ///
    /// # Returns
    ///
    /// - `f32` - Near intersection distance if hit, or `f32::INFINITY` on miss.
    #[inline(always)]
    #[must_use]
    pub(crate) fn intersect_near_fma(
        &self,
        inv: [f32; 3],
        neg_orig_inv: [f32; 3],
        t_min: f32,
        t_max: f32,
    ) -> f32 {
        let Self { min, max, .. } = self;
        let tx1: f32 = min[0].mul_add(inv[0], neg_orig_inv[0]);
        let tx2: f32 = max[0].mul_add(inv[0], neg_orig_inv[0]);
        let mut tmin: f32 = t_min.max(tx1.min(tx2));
        let mut tmax: f32 = t_max.min(tx1.max(tx2));
        let ty1: f32 = min[1].mul_add(inv[1], neg_orig_inv[1]);
        let ty2: f32 = max[1].mul_add(inv[1], neg_orig_inv[1]);
        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));
        let tz1: f32 = min[2].mul_add(inv[2], neg_orig_inv[2]);
        let tz2: f32 = max[2].mul_add(inv[2], neg_orig_inv[2]);
        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));
        if tmax >= tmin && tmax >= 0.0 {
            tmin
        } else {
            f32::INFINITY
        }
    }
}

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
        let g: BvhGeomTri = BvhGeomTri::from_triangle(self);
        g.intersect_raw(ray.get_origin(), ray.get_direction(), t_min, t_max)
    }
}

/// Binned Surface Area Heuristic (SAH) BVH construction and near-child-first ray traversal for `Bvh3D`.
impl Bvh3D {
    /// Builds a tight SAH flat BVH over `triangles` and populates its 32-byte DFS node and 36-byte geometry mirrors.
    ///
    /// # Arguments
    ///
    /// - `Vec<BvhTriangle>` - World-space triangles to index.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed SAH BVH acceleration structure.
    #[must_use]
    pub fn build(triangles: Vec<BvhTriangle>) -> Self {
        let tri_len: usize = triangles.len();
        let mut bvh: Self = Self {
            nodes: Vec::with_capacity(tri_len * 2 + 1),
            triangles,
            geom: Vec::with_capacity(tri_len),
            leaf_packets: Vec::with_capacity(tri_len.div_ceil(4) + 16),
            flat_nodes: Vec::with_capacity(tri_len * 2 + 1),
        };
        if !bvh.get_triangles().is_empty() {
            let total: usize = bvh.get_triangles().len();
            let _: u32 = bvh.build_node(0, total);
            let Self {
                nodes,
                triangles,
                geom,
                leaf_packets,
                flat_nodes,
            } = &mut bvh;
            geom.clear();
            for tri in triangles.iter() {
                geom.push(BvhGeomTri::from_triangle(tri));
            }
            leaf_packets.clear();
            flat_nodes.clear();
            for n in nodes.iter() {
                let mn: Vec3 = n.get_bounds().get_min();
                let mx: Vec3 = n.get_bounds().get_max();
                let tc: u32 = n.get_tri_count();
                if tc > 0 {
                    let first_tri: usize = n.get_first_tri() as usize;
                    let end_tri: usize = first_tri + (tc as usize);
                    let pkt_first: u32 = leaf_packets.len() as u32;
                    let mut cursor: usize = first_tri;
                    while cursor < end_tri {
                        let next_end: usize = (cursor + 4).min(end_tri);
                        leaf_packets.push(BvhTri4::from_geom_chunk(
                            &geom[cursor..next_end],
                            cursor as u32,
                        ));
                        cursor = next_end;
                    }
                    let pkt_count: u32 = (leaf_packets.len() as u32) - pkt_first;
                    flat_nodes.push(BvhFlatNode {
                        min: [mn.get_x(), mn.get_y(), mn.get_z()],
                        right_or_first: pkt_first,
                        max: [mx.get_x(), mx.get_y(), mx.get_z()],
                        tri_count: pkt_count,
                    });
                } else {
                    flat_nodes.push(BvhFlatNode {
                        min: [mn.get_x(), mn.get_y(), mn.get_z()],
                        right_or_first: n.get_right(),
                        max: [mx.get_x(), mx.get_y(), mx.get_z()],
                        tri_count: 0,
                    });
                }
            }
        }
        bvh
    }

    /// Extracts the `axis` coordinate (`0 = x`, `1 = y`, `2 = z`) from vector `v`.
    ///
    /// # Arguments
    ///
    /// - `Vec3` - Source 3D vector.
    /// - `usize` - Axis index.
    ///
    /// # Returns
    ///
    /// - `f32` - Selected scalar component.
    #[must_use]
    fn vec_axis(v: Vec3, axis: usize) -> f32 {
        match axis {
            0 => v.get_x(),
            1 => v.get_y(),
            _ => v.get_z(),
        }
    }

    /// Evaluates 12-bin Surface Area Heuristic splits across all three axes and returns `(best_axis, best_split_pos)`.
    ///
    /// # Arguments
    ///
    /// - `&[BvhTriangle]` - Active triangle range.
    /// - `Aabb3D` - Centroid bounding box of the range.
    /// - `f32` - Parent bounding-box surface area.
    ///
    /// # Returns
    ///
    /// - `Option<(usize, f32)>` - Optimal split axis and threshold if cheaper than forming a leaf.
    #[must_use]
    fn find_sah_split(
        tris: &[BvhTriangle],
        cent_bounds: Aabb3D,
        parent_area: f32,
    ) -> Option<(usize, f32)> {
        let bins: usize = 12;
        let extent: Vec3 = cent_bounds.get_max().sub(cent_bounds.get_min());
        let exts: [f32; 3] = [extent.get_x(), extent.get_y(), extent.get_z()];
        let cmins: [f32; 3] = [
            cent_bounds.get_min().get_x(),
            cent_bounds.get_min().get_y(),
            cent_bounds.get_min().get_z(),
        ];
        let active: [bool; 3] = [exts[0] >= 1e-5, exts[1] >= 1e-5, exts[2] >= 1e-5];
        let scales: [f32; 3] = [
            if active[0] {
                (bins as f32) / exts[0]
            } else {
                0.0
            },
            if active[1] {
                (bins as f32) / exts[1]
            } else {
                0.0
            },
            if active[2] {
                (bins as f32) / exts[2]
            } else {
                0.0
            },
        ];
        let mut bin_bounds: [[Aabb3D; 12]; 3] = [[Aabb3D::empty(); 12]; 3];
        let mut bin_counts: [[u32; 12]; 3] = [[0; 12]; 3];
        for tri in tris.iter() {
            let c: Vec3 = tri.get_centroid();
            let tb: Aabb3D = tri.get_bounds();
            if active[0] {
                let b0: usize = (((c.get_x() - cmins[0]) * scales[0]) as usize).min(bins - 1);
                bin_counts[0][b0] += 1;
                bin_bounds[0][b0].expand_box(tb);
            }
            if active[1] {
                let b1: usize = (((c.get_y() - cmins[1]) * scales[1]) as usize).min(bins - 1);
                bin_counts[1][b1] += 1;
                bin_bounds[1][b1].expand_box(tb);
            }
            if active[2] {
                let b2: usize = (((c.get_z() - cmins[2]) * scales[2]) as usize).min(bins - 1);
                bin_counts[2][b2] += 1;
                bin_bounds[2][b2].expand_box(tb);
            }
        }
        let mut best_cost: f32 = (tris.len() as f32) * parent_area;
        let mut best_split: Option<(usize, f32)> = None;
        for axis in 0..3 {
            if !active[axis] {
                continue;
            }
            let counts: &[u32; 12] = &bin_counts[axis];
            let bounds_arr: &[Aabb3D; 12] = &bin_bounds[axis];
            let mut left_area: [f32; 11] = [0.0; 11];
            let mut left_count: [u32; 11] = [0; 11];
            let mut acc_box: Aabb3D = Aabb3D::empty();
            let mut acc_cnt: u32 = 0;
            for i in 0..(bins - 1) {
                acc_cnt += counts[i];
                if counts[i] > 0 {
                    acc_box.expand_box(bounds_arr[i]);
                }
                left_count[i] = acc_cnt;
                left_area[i] = if acc_cnt > 0 {
                    acc_box.surface_area()
                } else {
                    0.0
                };
            }
            let mut r_box: Aabb3D = Aabb3D::empty();
            let mut r_cnt: u32 = 0;
            let step: f32 = exts[axis] / (bins as f32);
            for i in (1..bins).rev() {
                r_cnt += counts[i];
                if counts[i] > 0 {
                    r_box.expand_box(bounds_arr[i]);
                }
                let l_cnt: u32 = left_count[i - 1];
                if l_cnt == 0 || r_cnt == 0 {
                    continue;
                }
                let r_area: f32 = r_box.surface_area();
                let cost: f32 =
                    0.5 * parent_area + (l_cnt as f32) * left_area[i - 1] + (r_cnt as f32) * r_area;
                if cost < best_cost {
                    best_cost = cost;
                    let split_pos: f32 = cmins[axis] + (i as f32) * step;
                    best_split = Some((axis, split_pos));
                }
            }
        }
        best_split
    }

    /// Recursively partitions `triangles[start..end]` via binned SAH in DFS pre-order and appends a `BvhNode3D`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable BVH builder reference.
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
        if count <= 3 {
            return node_idx;
        }
        let parent_area: f32 = bounds.surface_area().max(1e-6);
        let split_opt: Option<(usize, f32)> =
            Self::find_sah_split(&self.get_triangles()[start..end], cent_bounds, parent_area);
        let mid: usize = if let Some((axis, split_pos)) = split_opt {
            let slice: &mut [BvhTriangle] = &mut self.get_mut_triangles()[start..end];
            let mut i: usize = 0;
            let mut j: usize = slice.len();
            while i < j {
                if Self::vec_axis(slice[i].get_centroid(), axis) < split_pos {
                    i += 1;
                } else {
                    j -= 1;
                    slice.swap(i, j);
                }
            }
            if i == 0 || i == count {
                start + count / 2
            } else {
                start + i
            }
        } else if count <= 6 {
            return node_idx;
        } else {
            let extent: Vec3 = cent_bounds.get_max().sub(cent_bounds.get_min());
            let axis: usize =
                if extent.get_x() >= extent.get_y() && extent.get_x() >= extent.get_z() {
                    0
                } else if extent.get_y() >= extent.get_z() {
                    1
                } else {
                    2
                };
            let mid_pos: f32 =
                Self::vec_axis(cent_bounds.get_min(), axis) + Self::vec_axis(extent, axis) * 0.5;
            let slice: &mut [BvhTriangle] = &mut self.get_mut_triangles()[start..end];
            let mut i: usize = 0;
            let mut j: usize = slice.len();
            while i < j {
                if Self::vec_axis(slice[i].get_centroid(), axis) < mid_pos {
                    i += 1;
                } else {
                    j -= 1;
                    slice.swap(i, j);
                }
            }
            if i == 0 || i == count {
                start + count / 2
            } else {
                start + i
            }
        };
        let left_idx: u32 = self.build_node(start, mid);
        let right_idx: u32 = self.build_node(mid, end);
        let node: &mut BvhNode3D = &mut self.get_mut_nodes()[node_idx as usize];
        node.set_left(left_idx);
        node.set_right(right_idx);
        node.set_tri_count(0);
        node_idx
    }

    /// Traces a shadow/occlusion ray using 32-byte DFS nodes, zero-division Möller–Trumbore, and a 1-triangle occluder cache.
    ///
    /// # Arguments
    ///
    /// - `&Self` - BVH reference.
    /// - `Vec3` - Ray origin.
    /// - `Vec3` - Unit ray direction.
    /// - `Vec3` - Precomputed safe reciprocal ray direction.
    /// - `f32` - Minimum offset (shadow bias).
    /// - `f32` - Maximum trace distance.
    /// - `&mut u32` - Per-thread last-occluding triangle index cache (`u32::MAX` if empty).
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if occluded by scene geometry.
    #[must_use]
    pub fn trace_any_shadow_cached(
        &self,
        origin: Vec3,
        dir: Vec3,
        inv_dir: Vec3,
        t_min: f32,
        t_max: f32,
        cached_tri: &mut u32,
    ) -> bool {
        let Self {
            flat_nodes,
            geom,
            leaf_packets,
            ..
        } = self;
        if flat_nodes.is_empty() || leaf_packets.is_empty() {
            return false;
        }
        let hint: usize = *cached_tri as usize;
        if hint < geom.len() && geom[hint].intersect_any_raw(origin, dir, t_min, t_max) {
            return true;
        }
        let orig_arr: [f32; 3] = [origin.get_x(), origin.get_y(), origin.get_z()];
        let dir_arr: [f32; 3] = [dir.get_x(), dir.get_y(), dir.get_z()];
        let inv_arr: [f32; 3] = [inv_dir.get_x(), inv_dir.get_y(), inv_dir.get_z()];
        let neg_orig_inv: [f32; 3] = [
            -orig_arr[0] * inv_arr[0],
            -orig_arr[1] * inv_arr[1],
            -orig_arr[2] * inv_arr[2],
        ];
        if !flat_nodes[0]
            .intersect_near_fma(inv_arr, neg_orig_inv, t_min, t_max)
            .is_finite()
        {
            return false;
        }
        let mut stack: [u32; 32] = [0; 32];
        let mut sp: usize = 1;
        while sp > 0 {
            sp -= 1;
            let idx: u32 = stack[sp];
            let n: &BvhFlatNode = &flat_nodes[idx as usize];
            let pkt_count: usize = n.tri_count as usize;
            if pkt_count > 0 {
                let first_pkt: usize = n.right_or_first as usize;
                for pkt in leaf_packets[first_pkt..(first_pkt + pkt_count)].iter() {
                    let hit_tri: u32 = pkt.intersect_any_4(orig_arr, dir_arr, t_min, t_max);
                    if hit_tri != u32::MAX {
                        *cached_tri = hit_tri;
                        return true;
                    }
                }
            } else {
                let left_idx: u32 = idx + 1;
                let right_idx: u32 = n.right_or_first;
                let tl: f32 = flat_nodes[left_idx as usize].intersect_near_fma(
                    inv_arr,
                    neg_orig_inv,
                    t_min,
                    t_max,
                );
                let tr: f32 = flat_nodes[right_idx as usize].intersect_near_fma(
                    inv_arr,
                    neg_orig_inv,
                    t_min,
                    t_max,
                );
                let hit_l: bool = tl.is_finite();
                let hit_r: bool = tr.is_finite();
                if hit_l && hit_r && sp + 2 <= 32 {
                    if tl <= tr {
                        stack[sp] = right_idx;
                        stack[sp + 1] = left_idx;
                    } else {
                        stack[sp] = left_idx;
                        stack[sp + 1] = right_idx;
                    }
                    sp += 2;
                } else if hit_l && sp < 32 {
                    stack[sp] = left_idx;
                    sp += 1;
                } else if hit_r && sp < 32 {
                    stack[sp] = right_idx;
                    sp += 1;
                }
            }
        }
        false
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
        let mut cache: u32 = u32::MAX;
        self.trace_any_shadow_cached(
            ray.get_origin(),
            ray.get_direction(),
            ray.get_inv_direction(),
            t_min,
            t_max,
            &mut cache,
        )
    }

    /// Traces a ray near-child-first against 32-byte DFS nodes and returns `(distance, tri_index)`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - BVH reference.
    /// - `Vec3` - Ray origin.
    /// - `Vec3` - Unit ray direction.
    /// - `Vec3` - Precomputed safe reciprocal ray direction.
    /// - `f32` - Minimum offset bias.
    /// - `f32` - Maximum trace distance.
    ///
    /// # Returns
    ///
    /// - `Option<(f32, usize)>` - Closest hit distance and triangle index into `self.triangles`.
    #[must_use]
    pub fn trace_closest_raw(
        &self,
        origin: Vec3,
        dir: Vec3,
        inv_dir: Vec3,
        t_min: f32,
        t_max: f32,
    ) -> Option<(f32, usize)> {
        let Self {
            flat_nodes,
            leaf_packets,
            ..
        } = self;
        if flat_nodes.is_empty() || leaf_packets.is_empty() {
            return None;
        }
        let orig_arr: [f32; 3] = [origin.get_x(), origin.get_y(), origin.get_z()];
        let dir_arr: [f32; 3] = [dir.get_x(), dir.get_y(), dir.get_z()];
        let inv_arr: [f32; 3] = [inv_dir.get_x(), inv_dir.get_y(), inv_dir.get_z()];
        let neg_orig_inv: [f32; 3] = [
            -orig_arr[0] * inv_arr[0],
            -orig_arr[1] * inv_arr[1],
            -orig_arr[2] * inv_arr[2],
        ];
        if !flat_nodes[0]
            .intersect_near_fma(inv_arr, neg_orig_inv, t_min, t_max)
            .is_finite()
        {
            return None;
        }
        let mut stack: [(u32, f32); 32] = [(0, 0.0); 32];
        let mut sp: usize = 1;
        let mut best_t: f32 = t_max;
        let mut best_idx: usize = usize::MAX;
        while sp > 0 {
            sp -= 1;
            let (node_id, node_tmin): (u32, f32) = stack[sp];
            if node_tmin >= best_t {
                continue;
            }
            let n: &BvhFlatNode = &flat_nodes[node_id as usize];
            let pkt_count: usize = n.tri_count as usize;
            if pkt_count > 0 {
                let first_pkt: usize = n.right_or_first as usize;
                for pkt in leaf_packets[first_pkt..(first_pkt + pkt_count)].iter() {
                    pkt.intersect_closest_4(orig_arr, dir_arr, t_min, &mut best_t, &mut best_idx);
                }
            } else {
                let left_idx: u32 = node_id + 1;
                let right_idx: u32 = n.right_or_first;
                let tl: f32 = flat_nodes[left_idx as usize].intersect_near_fma(
                    inv_arr,
                    neg_orig_inv,
                    t_min,
                    best_t,
                );
                let tr: f32 = flat_nodes[right_idx as usize].intersect_near_fma(
                    inv_arr,
                    neg_orig_inv,
                    t_min,
                    best_t,
                );
                let hit_l: bool = tl < best_t;
                let hit_r: bool = tr < best_t;
                if hit_l && hit_r && sp + 2 <= 32 {
                    if tl <= tr {
                        stack[sp] = (right_idx, tr);
                        stack[sp + 1] = (left_idx, tl);
                    } else {
                        stack[sp] = (left_idx, tl);
                        stack[sp + 1] = (right_idx, tr);
                    }
                    sp += 2;
                } else if hit_l && sp < 32 {
                    stack[sp] = (left_idx, tl);
                    sp += 1;
                } else if hit_r && sp < 32 {
                    stack[sp] = (right_idx, tr);
                    sp += 1;
                }
            }
        }
        if best_idx == usize::MAX {
            None
        } else {
            Some((best_t, best_idx))
        }
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
        let (t, idx): (f32, usize) = self.trace_closest_raw(
            ray.get_origin(),
            ray.get_direction(),
            ray.get_inv_direction(),
            t_min,
            t_max,
        )?;
        let tri: &BvhTriangle = &self.get_triangles()[idx];
        let pt: Vec3 = ray.get_origin().add(ray.get_direction().scale(t));
        Some(RayHit3D {
            distance: t,
            point: pt,
            normal: tri.get_normal(),
            material: tri.get_material(),
        })
    }
}
