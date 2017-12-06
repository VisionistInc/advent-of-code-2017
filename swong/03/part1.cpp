#include <iostream>
#include <math.h>
using namespace std;

struct Coordinates
{
  int x;
  int y;
};

Coordinates convertToCoordinates(int sqrtedNumber) {
  int startingX = 0;
  int startingY = 0;

  // if the roundedSqrt is even
  if (sqrtedNumber % 2 == 0) {
    startingX = -((sqrtedNumber/2)-1);
    startingY = (sqrtedNumber/2);
  } else {
    startingX = (sqrtedNumber-1)/2;
    startingY = -(sqrtedNumber-1)/2;
  }

  Coordinates initCoords = { startingX, startingY };

  return initCoords;
}

int main() {
  int inputNumber = 312051;
  Coordinates location;

  // take the sqrt of the inputNumber
  double sqrtedNumber = sqrt(inputNumber);

  if (sqrtedNumber == (int)sqrtedNumber) {
    // number is whole, find the coords
    location = convertToCoordinates(sqrtedNumber);
  } else {
    // number is not whole and needs to be rounded
    int roundedSqrt = round(sqrtedNumber);
    int doubledSqrt = roundedSqrt*roundedSqrt;
    location = convertToCoordinates(roundedSqrt);

    // if roundedSqrt is even
    if (roundedSqrt % 2 == 0) {
      // and we rounded up
      if (roundedSqrt > sqrtedNumber) {
        for (int i = 1; i < roundedSqrt; ++i) {
          location.x++;
          if (inputNumber == doubledSqrt-i) break;
        }
      } else { // otherwise if we rounded down
        // the first step is to the left on the x-axis
        location.x--;

        if (inputNumber != doubledSqrt+1) {
          // we step down on the y-axis
          for (int i = 1; i < roundedSqrt; ++i) {
            location.y--;
            if (inputNumber == doubledSqrt+i) break;
          }
        }
      }
    } else { // roundedSqrt is odd
      if (roundedSqrt > sqrtedNumber) {
        for (int i = 1; i < roundedSqrt; ++i) {
          location.x--;
          if (inputNumber == doubledSqrt-i) break;
        }
      } else {
        location.x++;

        if (inputNumber != doubledSqrt+1) {
          for (int i = 1; i < roundedSqrt; ++i) {
            location.y++;
            if (inputNumber == doubledSqrt+i) break;
          }
        }
      }
    }
  }

  cout << "X Location: " << location.x << "\n";
  cout << "Y Location: " << location.y << "\n";

  int distance = abs(location.x) + abs(location.y);

  cout << distance << "\n";
}
