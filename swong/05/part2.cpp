#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  vector<int>instructions;
  int currentLocation = 0;
  int steps = 0;

  while (getline(inputFile, currLine)) {
    int currNum;
    istringstream is(currLine);

    // store instructions in the vector
    while (is >> currNum) instructions.push_back(currNum);
  }

  // traverse the array until we go out of bounds
  while (currentLocation < instructions.size() || currentLocation < 0) {
    if (instructions[currentLocation] >= 3) currentLocation += instructions[currentLocation]--;
    else currentLocation += instructions[currentLocation]++;
    ++steps;
  }

  cout << steps << "\n";
}
