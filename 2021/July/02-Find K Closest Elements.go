package main

import "fmt"

func findClosestElements(arr []int, k int, x int) []int {

	if k == len(arr) {
		return arr
	} else {

		for i := 0; i < len(arr)-1; i++ {

			if arr[i]-x < arr[i+1]-x {

			}

			fmt.Println(arr[i])

		}

	}

	return arr
}

func main() {
	x := findClosestElements([]int{1, 2, 3, 4, 5}, 4, 3)
	fmt.Println(x)
}
