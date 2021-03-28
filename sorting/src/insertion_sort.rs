
pub fn sort(a: &mut [i32]) {
    for r in 1..a.len() {
        let mut l = r - 1; 
        let key = a[r]; 
        while a[l] > key {
            a[l + 1] = a[l]; 
            if l == 0 {
                break; 
            }
            l = l - 1; 
        }
        if l == 0 {
            a[l] = key; 
        } else {
            a[l + 1] = key; 
        }
    }
}