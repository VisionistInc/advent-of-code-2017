#include <iostream>
#include <fstream>
#include <sstream>
#include <unordered_map>
#include <regex>
using namespace std;

void updateMap(string value, unordered_map<string, int> &map) {
  pair<string, int> temp(value, 1);
  auto result = map.insert(temp);

  // if key already existed, increment value
  if (result.second == 0) {
    map[value]++;
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  regex pattern("(\\w+) \\(\\d+\\)(?: -> )?([a-zA-Z, ]+)?");
  smatch match;

  unordered_map<string, int>programMap;

  while (getline(inputFile, currLine)) {
    bool result = regex_search(currLine, match, pattern);

    if (result) {
      istringstream matched(match[2]);
      string group2;
      while (matched >> group2) {
        // remove trailing commas
        if (group2.back() == ',') {
          group2.pop_back();
        }

        updateMap(group2, programMap);
      }

      updateMap(match[1], programMap);
    }
  }

  // search map for a value equal to 1, this is the base
  for (auto it = programMap.begin(); it != programMap.end(); ++it) {
    if (it -> second == 1) cout << it -> first << "\n";
  }
}
