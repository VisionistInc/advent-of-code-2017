package main

// part one first try, part two sucks and I hope these aren't all
// math nerd problems

import (
	"fmt"
	"math"
)

func intDist(n1 int, n2 int) int {
	return int(math.Abs(float64(n1 - n2)))
}

func intDistWithin(n1 int, n2 int, dist int) bool {
	return intDist(n1, n2) <= dist
}

func main() {
	input := 361527

	// according to the algorithm presented, squares of ODD NUMBERS will
	// follow this path:
	// +-------+
	// |       |
	// |       |
	// |       |
	// |   X   | 1
	// |    X  | 9
	// |     X | 25
	// |      X| 49
	// +-------+
	//
	// approach: find the NEXT square number coordinate, work BACKWARDS
	// to the given input, and the "Manhattan Distance" is just the sum
	// of the absolute values of the coordinates |x|+|y|

	nextRoot := int(math.Ceil(math.Sqrt(float64(input))))
	if nextRoot%2 == 0 {
		nextRoot++ // must be odd
	}
	nextSquare := nextRoot * nextRoot

	// special case
	if nextSquare == input {
		fmt.Println(nextSquare)
		return
	}

	// working backwards: we're looking for an |x| and a |y|.  if we're
	// on the edge of a square, either |x| or |y| must equal half the edge.
	// the other coordinate will be the tangent distance from the midpoint
	// of the square side.  it doesn't matter which is x and which is y,
	// so we'll look for n1 and n2.
	var n1, n2 int

	// rounding down is desirable
	n1 = nextRoot / 2

	// the midpoints of the four sides are:
	// (remember integer div rounds down and nextRoot is always odd)
	southMidpoint := nextSquare - (nextRoot / 2)
	westMidpoint := southMidpoint - (nextRoot - 1)
	northMidpoint := westMidpoint - (nextRoot - 1)
	eastMidpoint := northMidpoint - (nextRoot - 1)

	// say the input is "on that side" if it's within nextRoot / 2
	// of a midpoint:
	maxMidpointDist := nextRoot / 2
	if intDistWithin(southMidpoint, input, maxMidpointDist) {
		n2 = intDist(southMidpoint, input)
	} else if intDistWithin(westMidpoint, input, maxMidpointDist) {
		n2 = intDist(westMidpoint, input)
	} else if intDistWithin(northMidpoint, input, maxMidpointDist) {
		n2 = intDist(northMidpoint, input)
	} else if intDistWithin(eastMidpoint, input, maxMidpointDist) {
		n2 = intDist(eastMidpoint, input)
	} else {
		panic("shouldn't be possible to get here")
	}

	fmt.Println(n1 + n2)

	// part two is a completely different problem >_<
	hateSequence := hateSpiral(7)
	for _, hateValue := range hateSequence {
		if hateValue > input {
			fmt.Println(hateValue)
			break
		}
	}
}
