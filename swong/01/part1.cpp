#include <iostream>
#include <fstream>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  char first;
  char curr;
  char next;
  int sum;

  while (inputFile.get(next)) {
    // need to save the first character for the wrap around
    if (!first) first = next;

    if (curr == next) sum += curr - '0';

    // prepare to read the next character
    if (next != '\n') curr = next;
  }

  // handle the wrap around case
  if (curr == first) sum += curr - '0';

  cout << sum << "\n";
}
