#include <iostream>
#include <vector>
using namespace std;

int main() {
  int inputSteps = 316;
  int currPosition = 0;
  vector<int>buffer;

  buffer.push_back(0);

  for (int i=1; i<=2017; ++i) {
    int insertIndex = (currPosition + inputSteps) % buffer.size() + 1;
    if (insertIndex > buffer.size() + 1) insertIndex = insertIndex - 1;
    buffer.insert(buffer.begin()+insertIndex, i);
    currPosition = insertIndex;
  }

  cout << buffer[currPosition+1] << "\n";
}
