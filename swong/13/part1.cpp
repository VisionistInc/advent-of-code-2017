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

void updateLayers(map<int, Layer>&firewall) {
  for (map<int,Layer>::iterator it=firewall.begin(); it!=firewall.end(); ++it) {
    Layer &currLayer = it->second;

    if (currLayer.range != 1) {
      if (currLayer.direction == 1) {
        if (currLayer.scanPos + 1 >= currLayer.range) {
          currLayer.direction = -1;
          currLayer.scanPos--;
        } else {
          currLayer.scanPos++;
        }
      } else {
        if (currLayer.scanPos - 1 < 0) {
          currLayer.direction = 1;
          currLayer.scanPos++;
        } else {
          currLayer.scanPos--;
        }
      }
    } //close if range != 1
  }
}

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

  int picoSecond = 0;
  int firewallEnd = firewall.rbegin()->first;
  int severity = 0;

  while (picoSecond <= firewallEnd) {
    Layer currLayer = firewall[picoSecond];
    if (currLayer.scanPos == 0) severity += currLayer.depth * currLayer.range;
    updateLayers(firewall);
    picoSecond++;
  }

  cout << severity << "\n";
}
