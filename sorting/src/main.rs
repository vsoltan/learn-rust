mod insertion_sort;
mod merge_sort; 
mod bubble_sort; 

fn main() {
    let mut insertion_arr = vec![3, 2, 1]; 

    println!("{:?}", insertion_arr);
    insertion_sort::sort(&mut insertion_arr);
    println!("{:?}", insertion_arr);

    let mut insertion_arr_2 = vec![42, 88, 43, 17]; 
    println!("{:?}", insertion_arr_2);
    insertion_sort::sort(&mut insertion_arr_2);
    println!("{:?}", insertion_arr_2);

    let mut bubble_arr = vec![3, 7, 1]; 

    println!("{:?}", bubble_arr);
    insertion_sort::sort(&mut bubble_arr);
    println!("{:?}", bubble_arr);

    let mut merge_sort_arr = vec![88, 43, 17]; 

    println!("{:?}", merge_sort_arr);
    let merged = merge_sort::sort(&mut merge_sort_arr);
    println!("{:?}", merged);

    // println!("{:?}", merge_sort::merge(vec![17, 43], vec![88]));

}
