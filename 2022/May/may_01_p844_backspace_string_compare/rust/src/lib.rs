struct Solution {}

struct BackSpacer {
    s: String,
    i: usize,
    cur: Option<char>,
}

impl BackSpacer {
    fn new(s: String) -> BackSpacer {
        let i = s.len() - 1;
        let cur = s.chars().nth(i);
        BackSpacer { s, i, cur }
    }

    fn move_back(&mut self) {
        if self.i == 0 {
            self.cur = None
        } else {
            self.i -= 1;
            self.cur = self.s.chars().nth(self.i);
        }
    }

    fn is_backspace(&self) -> bool {
        self.cur == Some('#')
    }

    fn trim_end(&mut self) {
        let mut c = 0;

        while self.cur.is_some() && (self.is_backspace() || c > 0) {
            if self.is_backspace() {
                c += 1;
            } else {
                c -= 1;
            }

            self.move_back();
        }
    }
}

impl Solution {
    pub fn backspace_compare_struct(s: String, t: String) -> bool {
        let mut bs1 = BackSpacer::new(s);
        let mut bs2 = BackSpacer::new(t);

        loop {
            bs1.trim_end();
            bs2.trim_end();

            match (bs1.cur, bs2.cur) {
                (None, None) => return true,

                (Some(c1), Some(c2)) => {
                    if c1 == c2 {
                        // same char, continue trimming
                        bs1.move_back();
                        bs2.move_back();
                    } else {
                        return false; // both are not empty and the chars are not same
                    }
                }
                (Some(_), None) => return false,
                (None, Some(_)) => return false,
            };
        }
    }

    

    pub fn backspace_compare(s: String, t: String) -> bool {

        fn get_char(s: &str, i: Option<usize>) -> Option<char> {
            if i.is_some() {
                s.chars().nth(i.unwrap())
            } else {
                None
            }
        }

        fn decrement_index(i: Option<usize>) -> Option<usize> {
            if let Some(x) = i {
                if x == 0 {
                    None
                } else {
                    Some(x - 1)
                }
            } else {
                None
            }
        }

        fn apply_backspaces(s: &str, i: Option<usize>) -> Option<usize> {
        
            let mut c = 0;
            let mut i = i;
    
            let mut sc = get_char(s, i);
    
            while i.is_some() && (sc == Some('#') || c > 0) {
    
                match sc {
                    Some('#') => c += 1,
                    _ => c -= 1,
                }

                i = decrement_index(i);
                sc = get_char(s, i);
    
            }
    
            i
    
        }

        fn get_index(s: &str) -> Option<usize> {
            if s.len() == 0 {
                None
            } else {
                Some(s.len() - 1)
            }
        }

        let mut si = get_index(&s);
        let mut ti = get_index(&t);

        loop {
            if si.is_none() && ti.is_none() {
                return true;
            } else {
                let sc = get_char(&s, si);
                let tc = get_char(&t, ti);

                if sc != Some('#') && tc != Some('#') {
                    if sc != tc {
                        return false;
                    }

                    si = decrement_index(si);
                    ti = decrement_index(ti);
                    
                } else {
                    si = apply_backspaces(&s, si);
                    ti = apply_backspaces(&t, ti);

                    if (si.is_none() && ti.is_some()) || (ti.is_none() && si.is_some()) {
                        return false;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::backspace_compare(
            "nzp#o#g".into(),
            "b#nzp#o#g".into()
        ));
        assert!(!Solution::backspace_compare(
            "bxj##tw".into(),
            "bxj###tw".into()
        ));
        assert!(Solution::backspace_compare(
            "bxj##tw".into(),
            "bxo#j##tw".into()
        ));
        assert!(Solution::backspace_compare("ab#c".into(), "ad#c".into()));
        assert!(Solution::backspace_compare("ab##".into(), "c#d#".into()));
        assert!(!Solution::backspace_compare("a#c".into(), "b".into()));
    }
}
