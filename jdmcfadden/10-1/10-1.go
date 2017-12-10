package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	valueString, err := ioutil.ReadFile("./input")
	if err != nil {
		panic(err)
	}

	valueStrings := strings.Split(string(valueString), ",")

	// parse the input into a list of lengths
	lengths := make([]int, len(valueStrings))
	for ii, valueString := range valueStrings {
		value, err := strconv.Atoi(strings.Trim(valueString, "\n"))
		if err != nil {
			panic(err)
		}

		lengths[ii] = value
	}

	// generate the initial state
	// numValues := 5
	numValues := 256
	values := make([]int, numValues)
	for ii := 0; ii < len(values); ii++ {
		values[ii] = ii
	}

	// lengths = []int{3, 4, 1, 5}
	fmt.Println(lengths)

	// carry out the hash
	var curPos, skip int
	for _, length := range lengths {

		// pull out a slice of length _length_ starting at _curPos_
		toReverse := make([]int, length)
		for ii := 0; ii < length; ii++ {
			valueIndex := (ii + curPos) % len(values)
			toReverse[ii] = values[valueIndex]
		}

		// reverse the slice back into the values
		for ii := 0; ii < length; ii++ {
			valueIndex := (ii + curPos) % len(values)
			values[valueIndex] = toReverse[length-ii-1]
		}

		curPos += length + skip
		skip++
	}

	fmt.Println(values[0] * values[1])
}
