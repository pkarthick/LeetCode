package May

type Backspacer struct {
	s string
	i int
}

func newBackSpacer(s string) Backspacer {
	return Backspacer{s, len(s) - 1}
}

func (b *Backspacer) trimEnd() (byte, bool) {

	for c := 0; b.i >= 0 && (b.s[b.i] == '#' || c > 0); b.i-- {
		if b.s[b.i] == '#' {
			c++
		} else {
			c--
		}
	}

	if b.i >= 0 {
		return b.s[b.i], true
	} else {
		return 0, false
	}
}

func (b *Backspacer) moveBack() {
	b.i--
}

func backspaceCompare(s string, t string) bool {

	bs1 := newBackSpacer(s)
	bs2 := newBackSpacer(t)

	for {

		c1, notEmpty1 := bs1.trimEnd()
		c2, notEmpty2 := bs2.trimEnd()

		switch {
		case !notEmpty1 && !notEmpty2:
			return true

		case notEmpty1 && notEmpty2:
			if c1 == c2 { // same char, continue trimming
				bs1.moveBack()
				bs2.moveBack()
			} else {
				return false // both are not empty and the chars are not same
			}
		case notEmpty1:
			if !notEmpty2 {
				return false
			}

		case notEmpty2:
			if !notEmpty1 {
				return false
			}
		}

	}
}

/*

func doBackspaces(s string, i *int) {

	for c := 0; *i >= 0 && (s[*i] == '#' || c > 0); *i-- {
		if s[*i] == '#' {
			c++
		} else {
			c--
		}
	}

}

func backspaceCompare_imperative(s string, t string) bool {

	si := len(s) - 1
	ti := len(t) - 1

	for {

		if si == -1 && ti == -1 {
			return true
		} else {

			if si >= 0 && s[si] != '#' && ti >= 0 && t[ti] != '#' {
				if s[si] != t[ti] {
					return false
				}
				si--
				ti--

			} else {

				doBackspaces(s, &si)
				doBackspaces(t, &ti)

				if (si == -1 && ti >= 0) || (ti == -1 && si >= 0) {
					return false
				}
			}

		}
	}
}

*/

func BackspaceCompare(s string, t string) bool {
	return backspaceCompare(s, t)
}
