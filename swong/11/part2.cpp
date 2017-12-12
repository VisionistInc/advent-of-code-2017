#include <iostream>
#include <fstream>
#include <sstream>
#include <cmath>
using namespace std;

struct Coordinates {
  double x;
  double y;
};

void parseDirection(string direction, Coordinates &coords) {
  if (direction == "n") coords.y++;
  else if (direction == "s") coords.y--;
  else if (direction == "ne") {
    coords.x++;
    coords.y += 0.5;
  } else if (direction == "nw") {
    coords.x--;
    coords.y += 0.5;
  } else if (direction == "sw") {
    coords.x--;
    coords.y -= 0.5;
  } else if (direction == "se") {
    coords.x++;
    coords.y -= 0.5;
  }
}

int calcSteps(Coordinates coords) {
  double x = coords.x;
  double y = coords.y;
  // ne of origin
  if (x > 0 && y > 0) return x + y-(x/2);
  // nw of origin
  else if (x < 0 && y > 0) return abs(x) + y-(abs(x)/2);
  // sw of origin
  else if (x < 0 && y < 0) return abs(x) + abs(y)-(abs(x)/2);
  // sw of origin
  else if (x > 0 && y < 0) return x + abs(y)-(x/2);
  else return 0;
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  string currDir;
  Coordinates coords = { 0, 0 };
  int maxDist = 0;

  while (getline(inputFile, currLine)) {
    istringstream ss(currLine);

    while (getline(ss, currDir, ',')) {
      // cout << currDir << "\n";
      parseDirection(currDir, coords);
      if (calcSteps(coords) > maxDist) maxDist = calcSteps(coords);
    }
  }

  cout << maxDist << "\n";
}
