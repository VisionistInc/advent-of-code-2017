#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <unordered_map>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  string input;
  vector<int>memoryBanks;
  unordered_map<string, int> map;
  int redistributionCycles = 0;

  while (inputFile >> input) {
    int currNum;
    istringstream is(input);

    // store initial memoryBanks state in the vector
    while (is >> currNum) memoryBanks.push_back(currNum);
  }

  while (true) {
    // get a snapshot of the current state and save to hash table
    stringstream currentState;
    copy(memoryBanks.begin(), memoryBanks.end(), ostream_iterator<int>(currentState, ","));

    pair<string, int> temp (currentState.str(), 1);
    auto insertResult = map.insert(temp);

    // if unable to insert to hash table, the state has been seen before
    if (insertResult.second == 0) {
      break;
    } else {
      // find the max value/location and allocate it's value to the other registers
      int position = distance(memoryBanks.begin(), max_element(memoryBanks.begin(), memoryBanks.end()));
      int max = memoryBanks[position];

      // reset the value of the max to 0
      memoryBanks[position] = 0;

      // traverse the array i times where i was the value at memoryBanks[position] (max)
      for (int i=0; i < max; ++i) {
        if (++position >= memoryBanks.size()) position = 0;
        memoryBanks[position]++;
      }

      ++redistributionCycles;
    }
  }

  cout << redistributionCycles << "\n";
}
