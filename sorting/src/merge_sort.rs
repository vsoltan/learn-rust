
// merges two slices into a vector 
pub fn merge(m: Vec<i32>, n: Vec<i32>) -> Vec<i32> {
    let mut num_inserted = 0; 
    let mut m_idx = 0; 
    let mut n_idx = 0; 

    let mut merged_vec : Vec<i32> = vec![]; 

    while num_inserted < m.len() && num_inserted < n.len() {
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
    println!("{:?}", v);
    let l = 0; 
    let r = v.len() - 1; 
    if r == 0 {
        return v.clone(); 
    }
    if r == 1 {
        if v[0] > v[1] {
            v.swap(0, 1); 
        }
        println!("{:?}", v);
        return v.clone(); 
    }
    let mid = (l + r) / 2; 
    let mut left = v.clone(); 
    let mut right = left.split_off(mid); 
    return merge(sort(&mut left),
                 sort(&mut right));
}


