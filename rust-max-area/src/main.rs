// find container that holds max area between any two lines along an axis
use std::cmp;

pub struct Containers {
    heights: Vec<i32>,
}

impl Containers {
    pub fn new(h: Vec<i32>) -> Self {
        Self { heights: h }
    }

    fn _area(&self, left_idx: usize, right_idx: usize) -> i32 {
        (right_idx as i32 - left_idx as i32)
            * cmp::min(self.heights[left_idx], self.heights[right_idx])
    }

    pub fn find_max_area(self) -> i32 {
        let mut max_area: i32 = 0;
        let mut left_idx: usize = 0;
        let mut right_idx: usize = self.heights.len() - 1;

        while left_idx < right_idx {
            max_area = cmp::max(max_area, self._area(left_idx, right_idx));

            match self.heights[left_idx] < self.heights[right_idx] {
                true => left_idx += 1,
                false => right_idx -= 1,
            }
        }
        max_area
    }
}

fn main() {
    let containers = Containers::new([1, 8, 6, 2, 5, 7].to_vec());
    let max_area = containers.find_max_area();

    println!("max area: {:?}", max_area);
    assert_eq!(max_area, 28);
}
