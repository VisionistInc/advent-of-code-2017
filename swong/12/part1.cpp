#include <iostream>
#include <fstream>
#include <sstream>
#include <regex>
#include <vector>
#include <set>
using namespace std;

void dfs(vector<vector<int>>inputs, int currNode, set<int> &visited) {
  visited.insert(currNode);
  for (int i=0; i<inputs[currNode].size(); ++i) {
    if (visited.find(inputs[currNode][i]) == visited.end()) {
      dfs(inputs, inputs[currNode][i], visited);
    }
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  regex pattern("(.*) <-> (.*)");
  smatch match;
  int currVal;
  vector<vector<int>>inputs;

  // create the map of all input values
  while (getline(inputFile, currLine)) {
    bool result = regex_search(currLine, match, pattern);
    if (result) {
      vector<int>temp;
      stringstream ss(match[2]);

      while (ss >> currVal) {
        temp.push_back(currVal);
        if (ss.peek() == ',') ss.ignore();
      }

      inputs.push_back(temp);
    }
  }

  set<int>visited;

  dfs(inputs, 0, visited);

  cout << visited.size() << "\n";
}
