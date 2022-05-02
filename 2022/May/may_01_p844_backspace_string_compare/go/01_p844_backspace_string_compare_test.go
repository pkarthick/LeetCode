package May

import (
	"testing"
)

func TestBackspaceCompare(tt *testing.T) {
	test := func(s string, t string, equal bool) {
		if BackspaceCompare(s, t) != equal {
			tt.Fatalf(`%q is not equal to %q`, s, t)
		}
	}

	test("nzp#o#g", "b#nzp#o#g", true)
	test("bxj##tw", "bxj###tw", false)
	test("bxj##tw", "bxo#j##tw", true)
	test("ab#c", "ad#c", true)
	test("ab##", "c#d#", true)
	test("a#c", "b", false)
}
