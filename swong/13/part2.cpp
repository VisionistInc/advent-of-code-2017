#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <map>
using namespace std;

struct Layer {
  int depth;
  int scanPos;
  int range;
  int direction;
};

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  map<int, Layer>firewall;

  // create the firewall
  while (getline(inputFile, currLine)) {
    string key = currLine.substr(0, currLine.find(": "));
    string layerLength = currLine.substr(currLine.find(": ") + 2);

    Layer layer = { stoi(key), 0, stoi(layerLength), 1 };

    pair<int, Layer> temp (stoi(key), layer);
    firewall.insert(temp);
  }

  bool safe = false;
  int delay = 0;

  while (!safe) {
    for (map<int,Layer>::iterator it=firewall.begin(); it!=firewall.end(); ++it) {
      safe = true;
      Layer currLayer = it->second;

      // conditions where our packet will not get through
      if ((delay + currLayer.depth) % ((currLayer.range-1) * 2) == 0) {
        safe = false;
        break;
      }
    }

    delay++;
  }
  cout << delay - 1 << "\n";
}
