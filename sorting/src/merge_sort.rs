fn merge(m: [i32], n: [i32]) {
    let mut merged = : [i32; m.len() + n.len()]; 
    let smaller_len : i32;
    if (m.len() > n.len()) {
        smaller_len = m.len();
    } else {
        smaller_len = n.len(); 
    }
    let m_idx: usize = 0;
    let n_idx: usize = 0; 
    let merged_idx : usize = 0; 
    for i in 0..smaller_len {
        if (m[m_idx] <= n[n_idx]) {
            merged[merged_idx] = m[m_idx]; 
            m_idx++; 
        } else {
            merged[merged_idx] = n[n_idx];
            n_idx++;
        }
        merged_idx++; 
    }
    return merged; 
}

pub fn sort(&a: [i32], l_idx : usize, r_idx : usize) {
    if ()
}
