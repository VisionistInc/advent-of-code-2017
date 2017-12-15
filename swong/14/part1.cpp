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

string hex_char_to_bin(char c) {
  switch(toupper(c))
  {
      case '0': return "0000";
      case '1': return "0001";
      case '2': return "0010";
      case '3': return "0011";
      case '4': return "0100";
      case '5': return "0101";
      case '6': return "0110";
      case '7': return "0111";
      case '8': return "1000";
      case '9': return "1001";
      case 'A': return "1010";
      case 'B': return "1011";
      case 'C': return "1100";
      case 'D': return "1101";
      case 'E': return "1110";
      case 'F': return "1111";
      default: return "!!!!";
  }
}


int main() {
  vector<string>knotHash;

  // run the knotHash algorithm 128 times
  for (int row=0; row<128; ++row) {
    string input = "oundnydw";
    vector<int>numbers;
    vector<int>lengths;
    int addLengths[] = {17, 31, 73, 47, 23};
    int currPos = 0;
    int skipSize = 0;

    // create the vector
    for (int i=0; i<=255; ++i) numbers.push_back(i);

    string inputRow = input + '-' + to_string(row);

    // create the list of lengths and append the hardcoded lengths
    for(char& currChar : inputRow) lengths.push_back(int(currChar));
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

    string hexHash;
    for (int i=0; i<denseHash.size(); ++i) hexHash += int_to_hex(denseHash[i]);
    knotHash.push_back(hexHash);
  } // end create 128x128 knotHash

  // convert the hex knotHash to binary
  for (int i=0; i<knotHash.size(); ++i) {
    string binary;

    for(char& hexChar : knotHash[i]) binary += hex_char_to_bin(hexChar);
    knotHash[i] = binary;
  }

  int numUsed = 0;

    // count the number of times a 1 shows up
  for (int i=0; i<knotHash.size(); ++i) {
    for(char& binaryDigit : knotHash[i]) {
      if (binaryDigit == '1') numUsed++;
    }
  }
  cout << numUsed << "\n";
}
