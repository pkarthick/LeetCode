package May

func sortArrayByParity_switch(nums []int) []int {
	e := len(nums) - 1
	s := 0

	for s < e {

		switch {

		case nums[s]%2 == 0:
			s++
		case nums[e]%2 == 1:
			e--
		default:
			nums[s], nums[e] = nums[e], nums[s]
			s++
			e--
		}

	}

	return nums
}

func sortArrayByParity(nums []int) []int {
	s := 0
	e := len(nums) - 1

	for s < e {
		if nums[s]&1 == 1 {
			if nums[e]&1 == 0 {
				nums[s], nums[e] = nums[e], nums[s]
				s++
			}
			e--
		} else {
			s++
		}
	}

	return nums
}

func SortArrayByParity(nums []int) []int {
	return sortArrayByParity(nums)
}
