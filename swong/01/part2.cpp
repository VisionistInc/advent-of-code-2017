#include <iostream>
#include <fstream>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  string line;
  int sum;
  int halfLength;

  while (getline(inputFile, line)) {
    halfLength = line.length()/2;

    // check first half against second half, sum matches
    for (int i = 0; i < halfLength; i++) {
      if (line[i] == line[i+halfLength]) sum += (line[i] - '0');
    }
  }
  
  // sum is the sum of half the values, double to account for other half
  cout << sum*2 << "\n";
}
