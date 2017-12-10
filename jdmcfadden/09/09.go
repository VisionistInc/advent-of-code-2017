package main

import (
	"fmt"
	"io/ioutil"
)

func main() {
	stream, err := ioutil.ReadFile("./input")
	if err != nil {
		panic(err)
	}

	// example inputs
	// stream = []byte("{{<a!>},{<a!>},{<a!>},{<ab>}}")
	// stream = []byte("<{o\"i!a,<{i<a>")

	var depth, sum, garbageCount int
	var inGarbage, skipNext bool

	for _, char := range stream {
		if inGarbage {
			// if we're skipping the next char, do so and reset
			if skipNext {
				skipNext = false
				continue
			}

			switch char {
			case '>':
				inGarbage = false
			case '!':
				skipNext = true
			default:
				garbageCount++
			}
		} else {
			switch char {
			case '{':
				depth++
				sum += depth
			case '<':
				inGarbage = true
			case '}':
				depth--
			default:
				// no-op, includes ','
			}
		}
	}

	fmt.Println(sum)
	fmt.Println(garbageCount)
}
