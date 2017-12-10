package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	lengths, err := ioutil.ReadFile("./input")
	if err != nil {
		panic(err)
	}
	// lengths := []byte("1,2,3")
	lengths = append([]byte(strings.TrimSpace(string(lengths))), []byte{17, 31, 73, 47, 23}...)

	// generate the initial state
	// numValues := 5
	numValues := 256
	values := make([]int, numValues)
	for ii := 0; ii < len(values); ii++ {
		values[ii] = ii
	}

	// carry out the hash
	var curPos, skip int
	for rounds := 0; rounds < 64; rounds++ {
		for _, length := range lengths {

			// pull out a slice of length _length_ starting at _curPos_
			toReverse := make([]int, length)
			for ii := 0; ii < int(length); ii++ {
				valueIndex := (ii + curPos) % len(values)
				toReverse[ii] = values[valueIndex]
			}

			// reverse the slice back into the values
			for ii := 0; ii < int(length); ii++ {
				valueIndex := (ii + curPos) % len(values)
				values[valueIndex] = toReverse[int(length)-ii-1]
			}

			curPos += int(length) + skip
			curPos = curPos % len(values)
			skip++
		}
	}

	// make the "dense hash"
	blockValues := make([]byte, 16)
	for block := 0; block < 16; block++ {
		for ii := 0; ii < 16; ii++ {
			blockValues[block] = blockValues[block] ^ byte(values[block*16+ii])
		}
	}

	for _, blockValue := range blockValues {
		fmt.Printf("%02x", blockValue)
	}
}
