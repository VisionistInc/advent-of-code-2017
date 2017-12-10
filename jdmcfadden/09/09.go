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

	// stream = []byte("{{<a!>},{<a!>},{<a!>},{<ab>}}")
	// stream = []byte("<{o\"i!a,<{i<a>")

	var depth, sum, garbageCount int
	var inGarbage bool
	var skipNext bool

	for _, char := range stream {
		if inGarbage {
			// if we're skipping the next char, do so and reset
			if skipNext {
				skipNext = false
				continue
			}

			if char == '>' {
				inGarbage = false
			} else if char == '!' {
				skipNext = true
			} else {
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
			case ',':
				// no-op
			default:
				// no-op
			}
		}
	}

	fmt.Println(sum)
	fmt.Println(garbageCount)
}
