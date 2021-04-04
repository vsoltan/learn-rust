
pub fn sort(v: &mut Vec<i32>) {
    for r in 1..v.len() {
        let mut l = r - 1; 
        let key = v[r]; 
        let mut oob = false; 
        while v[l] > key {
            v[l + 1] = v[l]; 
            if l == 0 {
                oob = true; 
                break; 
            }
            l -= 1; 
        }
        if oob {
            v[l] = key; 
        } else {
            v[l + 1] = key; 
        }
    }
}