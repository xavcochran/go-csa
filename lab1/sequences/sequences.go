package main

import "fmt"

func addOne(a int) int {
	return a + 1
}

func square(a int) int {
	return a * a
}

func double(slice* []int) {
	*slice = append(*slice, *slice...)
}

func mapSlice(f func(a int) int, slice []int) {
	for i, v := range slice {
		slice[i] = f(v)
	}
}

func mapArray(f func(a int) int, array* [3]int) {
	for i, v := range array {
		array[i] = f(v)
		fmt.Println(array[i],f(v))
	}
}

func main() {
	// slices are pointers to dynamic arrays so they are essentially passed by reference
	// think objects in javascript
	slice := []int{1, 2, 3, 4, 5}
	// mapSlice(addOne, slice)
	// fmt.Println(slice)
	// newSlice := slice[1:3]
	// mapSlice(square, newSlice)
	// fmt.Println(newSlice)
	// fmt.Println(slice)
	fmt.Println(slice)
	double(&slice)
	fmt.Println(slice)



	// arrays are the underlying value so they are passed by value
	intArray := [3]int{1, 2, 3}
	mapArray(addOne, &intArray)
	fmt.Println(intArray)
}
