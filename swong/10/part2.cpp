#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <iomanip>
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

template< typename T >
string int_to_hex( T i )
{
  stringstream stream;
  stream << setfill ('0') << setw(2)
         << hex << i;
  return stream.str();
}


int main() {
  ifstream inputFile("./input.txt");
  vector<int>numbers;
  vector<int>lengths;
  int addLengths[] = {17, 31, 73, 47, 23};
  int currPos = 0;
  int skipSize = 0;
  char currChar;

  // create the vector
  for (int i=0; i<=255; ++i) numbers.push_back(i);

  // create the list of lengths and append the hardcoded lengths
  while (inputFile >> currChar) lengths.push_back(int(currChar));
  for (int addLength: addLengths) lengths.push_back(addLength);

  // run 64 times
  for (int i=0; i < 64; ++i) {
    for (int j=0; j<lengths.size(); ++j) {
      reverse(currPos, lengths[j], numbers);

      currPos += lengths[j] + skipSize++;
      if (currPos > 255) currPos = currPos % 256;
    }
  }

  vector<int>denseHash;

  for (int i=0; i<16; ++i) {
    int xorResult = 0;
    for (int j=0; j<16; ++j) {
      xorResult ^= numbers[i*16+j];
    }
    denseHash.push_back(xorResult);
  }

  for (int i = 0; i< denseHash.size(); ++i) cout << int_to_hex(denseHash[i]);
  cout << "\n";
}
