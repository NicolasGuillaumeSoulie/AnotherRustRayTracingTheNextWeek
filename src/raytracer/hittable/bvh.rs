use super::{aabb::Aabb, HitRecord, Hittable, HittableList};
use crate::raytracer::Ray;
use rand::{rngs::ThreadRng, Rng};
use std::{cmp::Ordering, sync::Arc};

pub struct BvhNode {
    left: Arc<dyn Hittable + Sync + Send>,
    right: Arc<dyn Hittable + Sync + Send>,
    bounding_box: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(r, t_min, t_max, rec) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, t_max, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time_frame: (f64, f64)) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
impl BvhNode {
    pub fn new(
        list: &Vec<Arc<dyn Hittable + Send + Sync>>,
        rng: &mut ThreadRng,
        start: usize,
        end: usize,
        time_frame: &(f64, f64),
    ) -> BvhNode {
        let mut list = list.clone();
        let axis = rng.gen_range(0..3);
        let object_span = end - start;

        // let (left, right) =
        match object_span {
            1 => {
                return BvhNode {
                    right: list[0].clone(),
                    left: list[0].clone(),
                    bounding_box: Aabb::surronding_box(
                        &list[0].bounding_box(*time_frame).unwrap(),
                        &list[0].bounding_box(*time_frame).unwrap(),
                    ),
                }
            }
            // 2 => match Aabb::axis_compare(&list[start], &list[start + 1], axis) {
            //     Ok(true) => (&list[start], &list[start + 1]),
            //     Ok(false) => (&list[start + 1], &list[start]),
            //     _ => panic!(),
            // },
            _ => {
                list[start..end].sort_by(|a, b| -> Ordering {
                    if let (Some(box_a), Some(box_b)) =
                        (a.bounding_box((0.0, 0.0)), b.bounding_box((0.0, 0.0)))
                    {
                        return box_a.minimum[axis]
                            .partial_cmp(&box_b.minimum[axis])
                            .unwrap();
                    }
                    panic!();
                });
                let mid = start + object_span / 2;

                let left = Arc::new(BvhNode::new(&list, rng, start, mid, time_frame));
                let right = Arc::new(BvhNode::new(&list, rng, mid, end, time_frame));
                return BvhNode {
                    bounding_box: Aabb::surronding_box(
                        &left.bounding_box(*time_frame).unwrap(),
                        &right.bounding_box(*time_frame).unwrap(),
                    ),
                    left,
                    right,
                };
            }
        };

        todo!()
    }
}
