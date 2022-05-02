package May

import (
	"testing"
)

func evensFirstThenOdds(nums []int) bool {

	i := 0

	for ; i < len(nums); i++ {
		if nums[i]%2 == 1 {
			break
		}
	}

	for ; i < len(nums); i++ {
		if nums[i]%2 == 0 {
			return false
		}
	}

	return true
}

func TestSortParity(tt *testing.T) {

	if !evensFirstThenOdds(SortArrayByParity([]int{3, 1, 2, 4})) {
		tt.Fatalf("Test 1 failed!")
	}

	if !evensFirstThenOdds(SortArrayByParity([]int{0})) {
		tt.Fatalf("Test 2 failed!")
	}

	if !evensFirstThenOdds(SortArrayByParity([]int{1, 0, 3})) {
		tt.Fatalf("Test 3 failed!")
	}

}
