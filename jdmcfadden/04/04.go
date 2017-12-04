package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

func sortString(s string) string {
	chars := strings.Split(s, "")
	sort.Strings(chars)
	return strings.Join(chars, "")
}

func main() {

	doAnagrams := true

	input, err := os.Open("./input")
	if err != nil {
		panic(err)
	}
	lineScanner := bufio.NewScanner(input)
	lineScanner.Split(bufio.ScanLines)

	var countValid int

nextLine:
	for lineScanner.Scan() {
		line := lineScanner.Text()
		wordScanner := bufio.NewScanner(strings.NewReader(line))
		wordScanner.Split(bufio.ScanWords)

		words := make(map[string]struct{})
		for wordScanner.Scan() {
			word := wordScanner.Text()

			if doAnagrams {
				word = sortString(word)
			}

			if _, contains := words[word]; contains {
				continue nextLine
			}
			words[word] = struct{}{}
		}

		countValid++
	}

	fmt.Println(countValid)
}
