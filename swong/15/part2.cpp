#include <iostream>
using namespace std;

int main() {
  long int genA = 512;
  long int genB = 191;

  int genAFactor = 16807;
  int genBFactor = 48271;
  int divisor = 2147483647;

  int judgeCount = 0;

  // perform this 40 million times
  for (int i=0; i<5000000; ++i) {
    do {
      genA = (genA * genAFactor) % divisor;
    } while (genA % 4 != 0);
    do {
      genB = (genB * genBFactor) % divisor;
    } while (genB % 8 != 0);

    if ((genA & 0xffff) == (genB & 0xffff)) ++judgeCount;
  }
  cout << judgeCount << "\n";
}
