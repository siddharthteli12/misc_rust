use crate::sort::Sort;

struct MergeSort;


impl MergeSort {
    fn merge_sort(list: &[T], list2: &[T]) -> Vec<T> {

        
    }
}

impl<T: std::cmp::Ord> Sort<T> for MergeSort {
    fn sort(input: &mut [T]) {
        let merge_sort = MergeSort;
    }
}

struct BubbleSort;

impl<T: std::cmp::Ord + Copy> Sort<T> for BubbleSort {
    fn sort(input: &mut [T]) {
        for i in 0..input.len() {
            let mut flag = 0;
            for j in 0..(input.len() - i) {
                if input[j] > input[j + 1] {
                    input.swap(j, j + 1);
                    flag = 1;
                }
                if flag == 0 {
                    return;
                }
            }
        }
    }
}
