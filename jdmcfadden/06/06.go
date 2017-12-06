package main

import (
	"crypto/md5"
	"fmt"
	"sort"
)

// note: using a []byte for the banks because that's really easy to hash

// ReverseSortableBytes is a byte slice type that sorts from largest to smallest.
type ReverseSortableBytes []byte

// these functions implement sort.Interface:
func (s ReverseSortableBytes) Len() int {
	return len(s)
}
func (s ReverseSortableBytes) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}
func (s ReverseSortableBytes) Less(i, j int) bool {
	// backwards!
	return s[i] > s[j]
}

// md5 hash a byte slice as a string so it can be used as a map key:
func hash(bytes []byte) string {
	return fmt.Sprintf("%x", md5.Sum(bytes))
}

// returns the index of the largest byte in the slice:
func findIndexOfLargest(bytes []byte) int {
	// first, sort:
	sorted := append([]byte(nil), bytes...)
	sort.Sort(ReverseSortableBytes(sorted))
	largest := sorted[0]

	// then, find (first one wins):
	for ii := range bytes {
		if bytes[ii] == largest {
			return ii
		}
	}

	panic(fmt.Sprintf("Unable to find %d in %x", largest, bytes))
}

func main() {
	// example input for testing:
	// banks := []byte{0, 2, 7, 0}

	// day 06 input:
	banks := []byte{5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6}

	history := make([]string, 0)
	history = append(history, hash(banks))

	historyHashes := make(map[string]struct{})
	historyHashes[hash(banks)] = struct{}{}

	fmt.Println(banks)

	var count int
	for {
		// count once per redistribution
		count++

		// carry out redistribution
		distributeFrom := findIndexOfLargest(banks)
		toDistribute := banks[distributeFrom]
		banks[distributeFrom] = 0
		for ii := 0; ii < int(toDistribute); ii++ {
			distributeTo := (distributeFrom + 1 + ii) % len(banks)
			banks[distributeTo] = banks[distributeTo] + 1
		}

		fmt.Println(banks)

		// hash the new bank layout and deal with it:
		newHash := hash(banks)
		if _, contains := historyHashes[newHash]; contains {

			// we found a cycle, so measure it from the end of the history:
			for ii := 0; ii < len(history); ii++ {
				if history[len(history)-1-ii] == newHash {
					fmt.Printf("Cycle length: %d\n", ii+1)
					break
				}
			}

			break
		} else {
			// haven't found a cycle yet, so update history
			history = append(history, newHash)
			historyHashes[newHash] = struct{}{}
		}
	}

	fmt.Printf("Number of operations: %d\n", count)
}
