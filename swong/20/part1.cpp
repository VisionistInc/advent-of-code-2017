#include <iostream>
#include <fstream>
#include <sstream>
#include <regex>
#include <climits>
#include <cmath>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  string currVal;
  regex pattern("p=<(.*)>, v=<(.*)>, a=<(.*)>");
  smatch match;
  int accDist = INT_MAX;
  int currentParticle = 0;
  int closestParticle = -1;
  
  while (getline(inputFile, currLine)) {
    regex_search(currLine, match, pattern);
    int currValAcc = 0;
    
    istringstream ss(match[3]);
    while (getline(ss, currVal, ',')) {
      currValAcc += abs(stoi(currVal));
    }

    if (currValAcc < accDist) {
      accDist = currValAcc;
      closestParticle = currentParticle;
    }
    ++currentParticle;
  }
  cout << closestParticle << "\n";
}
