#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  int sum;
  
  while (getline(inputFile, currLine)) {
    vector<int>numbers;
    int currNum;
    istringstream is(currLine);

    // store the values for the current line in an array
    while(is >> currNum) {
      numbers.push_back(currNum);
    }

    for (int i=0; i<numbers.size() - 1; ++i) {
      for (int j=i+1; j<numbers.size(); ++j) {
        if (numbers[i] % numbers[j] == 0) sum += numbers[i] / numbers[j];
        else if (numbers[j] % numbers[i] == 0) sum += numbers[j] / numbers[i];
      }
    }
  }
  
  cout << sum << "\n";
}