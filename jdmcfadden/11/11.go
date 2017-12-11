package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strings"
)

func abs(n int) int {
	return int(math.Abs(float64(n)))
}

// implementation: 3d coordinates from
// http://keekerdc.com/2011/03/hexagon-grids-coordinate-systems-and-distance-calculations/

type coord struct {
	x int // e
	y int // nw
	z int // sw
}

func (c coord) plus(c2 coord) coord {
	return coord{
		x: c.x + c2.x,
		y: c.y + c2.y,
		z: c.z + c2.z}
}

// https://www.redblobgames.com/grids/hexagons/#distances
func (c coord) manhattanDistanceToOrigin() int {
	return (abs(c.x) + abs(c.y) + abs(c.z)) / 2
}

func newCoord(x, y, z int) coord {
	return coord{x: x, y: y, z: z}
}

func main() {
	input, err := ioutil.ReadFile("./input")
	if err != nil {
		panic(err)
	}

	// example input:
	// input = []byte("se,sw,se,sw,sw")
	// input = []byte("ne,ne,ne,ne,ne,sw")
	// input = []byte("ne,ne,sw,sw")
	// input = []byte("ne,ne,s,s")

	directions := make(map[string]coord)
	directions["n"] = newCoord(0, 1, -1)
	directions["ne"] = newCoord(1, 0, -1)
	directions["se"] = newCoord(1, -1, 0)
	directions["s"] = newCoord(0, -1, 1)
	directions["sw"] = newCoord(-1, 0, 1)
	directions["nw"] = newCoord(-1, 1, 0)

	moves := strings.Split(string(input), ",")

	position := newCoord(0, 0, 0)
	var maxDistance int
	for _, move := range moves {

		// boy howdy I got screwed by that last newline
		move = strings.TrimSpace(move)

		position = position.plus(directions[move])

		curDistance := position.manhattanDistanceToOrigin()
		if curDistance > maxDistance {
			maxDistance = curDistance
		}
	}

	distance := position.manhattanDistanceToOrigin()
	fmt.Println(distance)
	fmt.Println(maxDistance)
}
