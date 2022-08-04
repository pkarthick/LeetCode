package May

import "testing"

func TestFindUnsortedSubarray(t *testing.T) {

	count := findUnsortedSubarray([]int{1, 4, 2, 5, 3, 1, 4, 3, 99, 100, 101, 203, 222, 210, 150, 213})
	if count != 15 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{1, 5, 3, 2, 4})
	if count != 4 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{1, 3, 5, 4, 2})
	if count != 4 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{2, 3, 3, 2, 4})
	if count != 3 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{1, 3, 2, 3, 3})
	if count != 2 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{2, 6, 4, 8, 10, 9, 15})
	if count != 5 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{2, 6, 4, 8, 12, 10, 11})

	if count != 6 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{2, 6, 4, 8, 9, 10, 11})

	if count != 2 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{2, 6, 8, 9, 10, 11})

	if count != 0 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{2})

	if count != 0 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{2, 1})

	if count != 2 {
		t.Error("Failed")
	}

	count = findUnsortedSubarray([]int{-10, -8, 2, 1})

	if count != 2 {
		t.Error("Failed")
	}

}
