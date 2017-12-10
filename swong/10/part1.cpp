#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
using namespace std;

void reverse(int currPos, int currLength, vector<int> &numbers) {
  vector<int>subset;

  // create the subset that needs to be reversed
  for (int i=0; i<currLength; ++i) {
    int value = i + currPos;
    if (value > 255) value -= 256;
    subset.push_back(numbers[value]);
  }

  int temp;
  int subsetStart = 0;
  int subsetEnd = subset.size()-1;

  // reverse the subset
  while (subsetStart < subsetEnd) {
    temp = subset[subsetStart];
    subset[subsetStart] = subset[subsetEnd];
    subset[subsetEnd] = temp;
    subsetStart++;
    subsetEnd--;
  }

  // insert the reversed subset into the original vector
  for (int i=0; i<subset.size(); ++i) {
    int insertPos = currPos + i;
    if (insertPos > 255) insertPos -= 256;
    numbers[insertPos] = subset[i];
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  vector<int>numbers;
  int currPos = 0;
  int skipSize = 0;
  int currLength = 0;

  // create the vector
  for (int i=0; i<=255; ++i) numbers.push_back(i);

  while (getline(inputFile, currLine)) {
    stringstream ss(currLine);
    while (ss >> currLength) {
      reverse(currPos, currLength, numbers);

      currPos += currLength + skipSize++;
      if (currPos > 255) currPos = currPos % 256;

      // get the next length
      if (ss.peek() == ',') ss.ignore();
    }
  }

  cout << numbers[0] * numbers[1] << "\n";
}
