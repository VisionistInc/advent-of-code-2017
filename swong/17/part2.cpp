#include <iostream>
using namespace std;

int main() {
  int inputSteps = 316;
  int currPosition = 0;
  int bufferSize = 1;
  int mostRecentlyInsertedAtPositionOne = 0;

  // we don't actually need to create the array
  for (int i=1; i<=50000000; ++i) {
    // the only way it will insert in buffer[1] is:
    if ((currPosition + inputSteps) % bufferSize == 0) mostRecentlyInsertedAtPositionOne = i;

    // update the current position and bufferSize
    // note the +1, we insert to position X, but currPosition is actually the location of the new element and not the insertion location
    currPosition = ((currPosition + inputSteps) % bufferSize) + 1;
    ++bufferSize;
  }

  cout << mostRecentlyInsertedAtPositionOne << "\n";
}
