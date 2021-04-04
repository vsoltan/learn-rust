
pub fn sort(a: &mut Vec<i32>) {
    if a.len() <= 1 {
        return;
    }
    for i in 0..a.len() - 1 {
        for j in i..a.len() {
            if a[i] > a[j] {
                a.swap(i, j);
            }
        }
    }
}