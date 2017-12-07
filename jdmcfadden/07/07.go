package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"

	"github.com/twmb/algoimpl/go/graph"
)

// I made three horrible mistakes:
// 1. chose a weaksauce graph impl... is there a better one?
// 2. started implementing "weight" as edge weight.  it's NODE WEIGHT!
// 3. didn't make a proper data structure for the nodes, which made calculations in part 2 super freakin awkward

// recursive function to aggregate the weights of all the nodes in the list
func collectWeight(g *graph.Graph, nodeNameToWeight map[string]int, nodes ...graph.Node) int {
	if len(nodes) == 0 {
		return 0
	}

	var weight int
	for _, node := range nodes {
		nodeName, _ := (*(node.Value)).(string)
		weight += nodeNameToWeight[nodeName]
		weight += collectWeight(g, nodeNameToWeight, g.Neighbors(node)...)
	}
	return weight
}

func main() {
	// create the digraph and a couple of lookup tables
	g := graph.New(graph.Directed)
	programs := make(map[string]graph.Node, 0)
	nodeNameToWeight := make(map[string]int, 0)

	// used to extract source node and node weight
	sourceRe := regexp.MustCompile("^([a-z]+)\\s\\((\\d+)\\)")

	input, err := os.Open("./input")
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	// parse the input into graph and LUTs
	for scanner.Scan() {
		line := scanner.Text()
		halves := strings.Split(line, " -> ")

		sourceValues := sourceRe.FindStringSubmatch(halves[0])
		sourceNode := sourceValues[1]
		weight, _ := strconv.Atoi(sourceValues[2])

		// weights are only listed on source nodes (not sink nodes)
		nodeNameToWeight[sourceNode] = weight

		// don't make a new node if it already exists
		if _, contains := programs[sourceNode]; !contains {
			programs[sourceNode] = g.MakeNode()
		}

		if len(halves) > 1 {
			sinkValues := strings.Split(halves[1], ", ")
			for _, sinkNode := range sinkValues {
				// don't make a new node if it already exists
				if _, contains := programs[sinkNode]; !contains {
					programs[sinkNode] = g.MakeNode()
				}

				// make a weighted edge
				g.MakeEdge(programs[sourceNode], programs[sinkNode])
			}
		}

	}

	// set the value pointer of each node to the map key for that node
	// (required for graph API)
	for key, node := range programs {
		*node.Value = key
	}

	// sorts with root node first and leaf nodes last
	sorted := g.TopologicalSort()
	fmt.Printf("Root node: %s\n", *(sorted[0].Value))

	// "sorted" is root-first -- traverse leaf-first, starting at the end
	for ii := len(sorted) - 1; ii >= 0; ii-- {
		node := sorted[ii]
		neighbors := g.Neighbors(node)
		if len(neighbors) == 0 {
			continue
		}

		accumWeightCounts := make(map[int]int)
		neighborAccumWeights := make(map[string]int)
		for _, neighbor := range neighbors {
			accumWeight := collectWeight(g, nodeNameToWeight, neighbor)
			neighborName, _ := (*(neighbor.Value)).(string)
			neighborAccumWeights[neighborName] = accumWeight
			if _, contains := accumWeightCounts[accumWeight]; !contains {
				accumWeightCounts[accumWeight] = 1
			} else {
				accumWeightCounts[accumWeight] = accumWeightCounts[accumWeight] + 1
			}
		}

		// for a node, if there are exactly two counts of accumulated values of weights of programs on that node, there must be an imbalance
		if len(accumWeightCounts) == 2 {
			// getting the two keys is annoying...
			twoWeights := make([]int, 2)
			var ii int
			for weight := range accumWeightCounts {
				twoWeights[ii] = weight
				ii++
			}

			for _, neighbor := range neighbors {
				neighborName, _ := (*(neighbor.Value)).(string)
				neighborWeight := nodeNameToWeight[neighborName]
				neighborAccumWeight := neighborAccumWeights[neighborName]
				fmt.Printf("Node %s of weight %d has %d total weight\n", neighborName, neighborWeight, neighborAccumWeight)
			}

			// whichever one there's only one of in the accumulated counts, take the difference from it:
			var difference int
			if accumWeightCounts[twoWeights[0]] == 1 {
				difference = twoWeights[1] - twoWeights[0]
			} else {
				difference = twoWeights[0] - twoWeights[1]
			}

			fmt.Printf("Adjust by: %d", difference)

			// I gave up at this point... without a better data structure it's very awkward to say "which is the accumulated node weight which there is only one of" and "which is the accumulated node weight which there is MORE THAN ONE of" and tie that back to individual node values.  I solved the problem by eyeballing the printed output.

			// bail as soon as we have reported on the leafmost imbalance
			return
		}
	}
}
