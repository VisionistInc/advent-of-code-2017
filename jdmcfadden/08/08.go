package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type instruction struct {
	Register          string
	Instruction       string
	Amount            int
	ConditionRegister string
	ConditionOperator string
	ConditionAmount   int
}

func main() {
	input, err := os.Open("./input")
	if err != nil {
		panic(err)
	}

	instructionRe := regexp.MustCompile("^([a-z]+) (inc|dec) ([0-9-]+) if ([a-z]+) ([!=<>]+) ([0-9-]+)$")

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	// parse the input into instructions
	instructions := make([]instruction, 0)
	for scanner.Scan() {
		line := scanner.Text()
		instructionParts := instructionRe.FindStringSubmatch(line)

		amount, _ := strconv.Atoi(instructionParts[3])
		conditionAmount, _ := strconv.Atoi(instructionParts[6])

		instruction := instruction{
			Register:          instructionParts[1],
			Instruction:       instructionParts[2],
			Amount:            amount,
			ConditionRegister: instructionParts[4],
			ConditionOperator: instructionParts[5],
			ConditionAmount:   conditionAmount}

		instructions = append(instructions, instruction)
	}

	var maxValueAtAnyTime int
	registers := make(map[string]int)
	for _, instruction := range instructions {
		// initialize new registers
		if _, contains := registers[instruction.Register]; !contains {
			registers[instruction.Register] = 0
		}
		if _, contains := registers[instruction.ConditionRegister]; !contains {
			registers[instruction.ConditionRegister] = 0
		}

		var condition bool
		switch instruction.ConditionOperator {
		case "<":
			condition = registers[instruction.ConditionRegister] < instruction.ConditionAmount
		case ">":
			condition = registers[instruction.ConditionRegister] > instruction.ConditionAmount
		case "==":
			condition = registers[instruction.ConditionRegister] == instruction.ConditionAmount
		case ">=":
			condition = registers[instruction.ConditionRegister] >= instruction.ConditionAmount
		case "<=":
			condition = registers[instruction.ConditionRegister] <= instruction.ConditionAmount
		case "!=":
			condition = registers[instruction.ConditionRegister] != instruction.ConditionAmount
		default:
			panic(fmt.Sprintf("Unknown operator: %s", instruction.ConditionOperator))
		}

		// if the condition is not met, do nothing
		if !condition {
			continue
		}

		switch instruction.Instruction {
		case "inc":
			registers[instruction.Register] = registers[instruction.Register] + instruction.Amount
		case "dec":
			registers[instruction.Register] = registers[instruction.Register] - instruction.Amount
		default:
			panic(fmt.Sprintf("Unknown instruction: %s", instruction.Instruction))
		}

		if registers[instruction.Register] > maxValueAtAnyTime {
			maxValueAtAnyTime = registers[instruction.Register]
		}
	}

	var maxValue int
	for _, value := range registers {
		if value > maxValue {
			maxValue = value
		}
	}

	fmt.Println(maxValue)
	fmt.Println(maxValueAtAnyTime)
}
