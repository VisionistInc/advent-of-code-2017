#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <set>
#include <algorithm>
using namespace std;

struct Component {
  int first;
  int second;
  bool visited;
};

void dfs(vector<Component>&components, int &maxTotal, int currOpenPort, int currTotal) {
  maxTotal = max(maxTotal, currTotal);

  for (int i=0; i<components.size(); ++i) {
    Component &currComponent = components[i];

    if (!currComponent.visited && (currComponent.first == currOpenPort || currComponent.second == currOpenPort)) {
      currComponent.visited = true;
      dfs(components, maxTotal, (currComponent.first == currOpenPort) ? currComponent.second : currComponent.first, currTotal + currComponent.first + currComponent.second);
      currComponent.visited = false;
    }
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  char temp;
  int first, second;
  int maxTotal = 0;
  vector<Component>components;
  
  while (getline(inputFile, currLine)) {
    istringstream ss(currLine);
    ss >> first >> temp >> second;

    Component component = { first, second, false };
    components.push_back(component);
  }
  
  dfs(components, maxTotal, 0, 0);
  
  cout << maxTotal << "\n";
}
