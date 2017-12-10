#include <iostream>
#include <fstream>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  bool inGarbage = false;
  char prev;
  char curr;
  int total = 0;
  int currVal = 0;

  while (inputFile.get(curr)) {
    if (inGarbage) {
      if (prev != '!' && curr == '>') inGarbage = false;
      // otherwise ignore, all other chars are garbage
    } else { // while not in a garbage sequence
      if (curr == '<') inGarbage = true;
      else if (curr == '{') currVal++;
      else if (curr == '}') total += currVal--;
    }
    // special case where !! negate each other out
    if (prev == '!' && curr == '!') prev = '\0';
    else prev = curr;
  }
  cout << total << "\n";
}
