package main

import (
	"encoding/csv"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
)

func main() {
	inputfile, _ := os.Open("./input")
	csvreader := csv.NewReader(inputfile)
	csvreader.Comma = '\t'
	sheet, _ := csvreader.ReadAll()

	// parse all the strings
	var numbers [][]float64
	for _, row := range sheet {
		var parsedrow []float64
		for _, cell := range row {
			value, _ := strconv.ParseFloat(cell, 64)
			parsedrow = append(parsedrow, value)
		}
		numbers = append(numbers, parsedrow)
	}

	var cksum float64
	for _, row := range numbers {
		min := math.MaxFloat64
		max := -min
		for _, number := range row {
			min = math.Min(min, number)
			max = math.Max(max, number)
		}
		diff := max - min
		cksum += diff
	}
	fmt.Println(cksum)

	// for each number in a sorted row, divide the remaining sorted numbers by
	// that number and bail on accumulating fmod by 1.0 = 0.0
	var divsum float64
	for _, row := range numbers {
		sort.Float64s(row)
		var n1, n2 float64
		for ii, number := range row {
		lookingfordivisor:
			for _, greaternumber := range row[ii+1:] {
				if math.Mod(greaternumber/number, 1.0) == 0.0 {
					n1 = number
					n2 = greaternumber
					divsum += (n2 / n1)
					break lookingfordivisor
				}
			}
		}
	}
	fmt.Println(divsum)

}
