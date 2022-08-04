package May

import "testing"

func TestMaxOperations(t *testing.T) {
	if maxOperations([]int{1, 2, 3, 4}, 5) != 2 {
		t.Error("Failed!")
	}

	if maxOperations([]int{3, 1, 3, 4, 3}, 6) != 1 {
		t.Error("Failed!")
	}

	if maxOperations([]int{3}, 3) != 0 {
		t.Error("Failed!")
	}

	if maxOperations([]int{3, 1, 3, 4, 3}, 8) != 0 {
		t.Error("Failed!")
	}
}
