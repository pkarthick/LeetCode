package May

import "sort"

func maxOperations(nums []int, k int) int {

	sort.Ints(nums)

	s := 0
	e := len(nums) - 1
	c := 0

	for s < e {
		if nums[s] == 0 || nums[s]+nums[e] < k {
			s++
		} else if nums[e] == 0 || nums[s]+nums[e] > k {
			e--
		} else {
			c++
			nums[s] = 0
			nums[e] = 0
			s++
			e--
		}
	}

	return c

}
