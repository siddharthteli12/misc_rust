pub trait Sort<T: std::cmp::Ord> {
    fn sort(input: &mut [T]);
}
