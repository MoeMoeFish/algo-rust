fn find_kmp(s: &str, prefixs: &Vec<&str>) -> usize {
    let len = s.len();
    for i in 1..len {
        let suff = &s[i..len];
        let suff_r = prefixs[len - i - 1];
        if suff == suff_r {
            return i;
        }
    }
    0
}

fn find_kmp_2(s: &str) -> usize {
    let len = s.len();
    if len == 0 || len == 1 {
        return 0;
    }

    let mut ret = 0;
    for i in 0..len - 1 {
        let prefix = &s[0..i + 1];
        let suff = &s[len - i - 1..len];

        if prefix == suff {
            ret = i + 1;
        }
    }

    ret
}

pub(crate) fn kmp1(s: &str, t: &str) -> bool {
    if s.len() < t.len() {
        return false;
    }

    let mut k_arr: Vec<usize> = vec!(); 

    // let mut prefixs: Vec<&str> = vec!();
    for i in 0..t.len() {
        let s = &t[0..i + 1];
        k_arr.push(find_kmp_2(s));
    }

    let mut i = 0;
    loop {
        if s.len() - i < t.len() {
            break;
        }

        let mut success = true;
        let mut br  = 0;
        for j in 0..t.len() {
            if s[i+j..i+j+1] != t[j..j+1] {
                success = false;
                br = j;
                break;
            }
        }
        
        if success {
            return true;
        }

        if br == 0 {
            i = i + 1;
        } else {
            i = i + br - k_arr[br];
        }
    }
    false 
}

pub(crate) fn demo() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kmp_2() {
        assert_eq!(find_kmp_2("abcd"), 0);
        assert_eq!(find_kmp_2("abcda"), 1);
        assert_eq!(find_kmp_2("abcdab"), 2);
    }

    #[test]
    fn test_kmp1() {
        assert!(kmp1("BBC ABCDAB ABCDABCDABDE", "ABCDABD"));
        assert_eq!(kmp1("BBC ABCDAB ABCDABCDABDE", "ABCDABDF"), false);
    }
}