#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
struct BackSpacer {
    s: String,
    i: usize,
    cur: Option<char>,
}

#[allow(dead_code)]
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

enum Index {
    OutOfRange,
    InRange(usize),
}

impl Index {
    fn new(s: &str) -> Self {
        if s.len() == 0 {
            Index::OutOfRange
        } else {
            Index::InRange(s.len() - 1)
        }
    }

    fn is_in_range(&self) -> bool {

        match self {
            Index::OutOfRange => false,
            Index::InRange(_) => true,
        }

    }

}

#[allow(dead_code)]
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

        fn get_char(s: &str, i: &Index) -> Option<char> {
            match i {
                Index::InRange(x) => s.chars().nth(*x),
                Index::OutOfRange => None
            }
        }

        fn decrement_index(i: &mut Index) {
            *i = match *i {
                Index::OutOfRange => Index::OutOfRange,
                Index::InRange(x) => if x == 0_usize {Index::OutOfRange} else { Index::InRange(x - 1) }
            };
        }

        fn apply_backspaces(s: &str, i: &mut Index) {
            let mut c = 0;

            let mut sc = get_char(s, i);

            while i.is_in_range() && (sc == Some('#') || c > 0) {
                match sc {
                    Some('#') => c += 1,
                    _ => c -= 1,
                }

                decrement_index(i);
                sc = get_char(s, i);
            }
        }


        let mut si = Index::new(&s);
        let mut ti = Index::new(&t);

        loop {
            if !si.is_in_range() && !ti.is_in_range() {
                return true;
            } else {
                let sc = get_char(&s, &si);
                let tc = get_char(&t, &ti);

                if sc != Some('#') && tc != Some('#') {
                    if sc != tc {
                        return false;
                    }

                    decrement_index(&mut si);
                    decrement_index(&mut ti);
                } else {
                    apply_backspaces(&s, &mut si);
                    apply_backspaces(&t, &mut ti);

                    if (!si.is_in_range() && ti.is_in_range()) || (!ti.is_in_range() && si.is_in_range()) {
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
