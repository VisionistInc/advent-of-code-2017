#include <iostream>
#include <fstream>
#include <sstream>
#include <climits>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  int sum;
  
  while (getline(inputFile, currLine)) {
    istringstream is(currLine);
    int currNum;
    int min = INT_MAX;
    int max = INT_MIN;

    while(is >> currNum) {
      if (currNum < min) min = currNum;
      if (currNum > max) max = currNum;
    }
    
    sum += max - min;
  }
  
  cout << sum << "\n";
}