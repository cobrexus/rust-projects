fn main() {}

fn sort_asc<T: PartialOrd>(list: &mut [T]) {
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..list.len() - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

fn sort_desc<T: PartialOrd>(list: &mut [T]) {
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..list.len() - 1 {
            if list[i] < list[i + 1] {
                list.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}
