package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	followRuleOfThree := true

	input, err := os.Open("./input")
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	var instructions []int
	for scanner.Scan() {
		line := scanner.Text()
		instruction, _ := strconv.Atoi(line)
		instructions = append(instructions, instruction)
	}

	var max = len(instructions)
	var ptr, count, instruction int
	for {
		instruction = instructions[ptr]
		if followRuleOfThree && instruction < 3 {
			instructions[ptr] = instructions[ptr] + 1
		} else {
			instructions[ptr] = instructions[ptr] - 1
		}
		ptr += instruction
		count++

		if ptr < 0 || ptr >= max {
			break
		}
	}

	fmt.Println(count)
}
