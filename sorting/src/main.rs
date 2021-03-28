mod insertion_sort;

fn main() {
    let mut a : [i32;3] = [3, 2, 1]; 
    println!("{:?}", a);
    insertion_sort::sort(& mut a);
    println!("{:?}", a);
}
