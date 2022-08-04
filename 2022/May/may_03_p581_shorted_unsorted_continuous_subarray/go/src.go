package May

func findUnsortedSubarray(nums []int) int {

	s := 0

	// find the starting index where the array is unsorted
	for ; s < len(nums)-1 && nums[s] <= nums[s+1]; s++ {
	}

	// starting index is the end index. so, array is sorted
	if s == len(nums)-1 {
		return 0
	} else {

		e := len(nums) - 1

		// find the starting index where the array is unsorted
		for ; nums[e-1] <= nums[e]; e-- {

		}

		// find the min and max between the start and end indices
		min := nums[s+1]
		max := nums[e]

		for j := s; j <= e; j++ {
			if nums[j] < min {
				min = nums[j]
			} else if nums[j] > max {
				max = nums[j]
			}
		}

		// traverse back until min or lower number is found
		for ; s >= 0 && nums[s] > min; s-- {
		}

		// traverse till max or higher number is found
		for ; e < len(nums) && nums[e] < max; e++ {
		}

		return e - (s + 1)

	}

}
