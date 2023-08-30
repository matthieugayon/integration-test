#[derive(Copy, Clone, Debug)]
pub enum SliderMove {
    Default,
    Relative(f32),
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum SliderStatus {
    Moved,
    #[default]
    Unchanged,
}

pub fn find_closest(value: f32, candidates: &Vec<f32>) -> (usize, &f32) {
    candidates
        .into_iter()
        .enumerate()
        .min_by(|(_, x), (_, y)| (value - *x).abs().partial_cmp(&(value - *y).abs()).unwrap())
        .unwrap()
}
