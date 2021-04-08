
// merges two slices into a vector 
pub fn merge(m: Vec<i32>, n: Vec<i32>) -> Vec<i32> {
    let mut num_inserted = 0; 
    let mut m_idx = 0; 
    let mut n_idx = 0; 

    let mut merged_vec : Vec<i32> = vec![]; 

    while m_idx < m.len() && n_idx < n.len() {
        if m[m_idx] <= n[n_idx] {
            merged_vec.push(m[m_idx]); 
            m_idx += 1; 
        } else {
            merged_vec.push(n[n_idx]); 
            n_idx += 1; 
        }
        num_inserted += 1; 
    }

    while m_idx < m.len() {
        merged_vec.push(m[m_idx]);
        m_idx += 1; 
    }

    while n_idx < n.len() {
        merged_vec.push(n[n_idx]);
        n_idx += 1; 
    }
    merged_vec 
}

pub fn sort(v: &mut Vec<i32>) -> Vec<i32> {
    let r = v.len() - 1; 
    if r == 0 {
        return v.clone(); 
    }
    if r == 1 {
        if v[0] > v[1] {
            v.swap(0, 1); 
        }
        return v.clone(); 
    }
    let mut left = v.clone(); 
    let mut right = left.split_off(r / 2); 
    return merge(sort(&mut left), sort(&mut right));
}


