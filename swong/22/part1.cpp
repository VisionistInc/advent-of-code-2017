#include <iostream>
#include <fstream>
#include <vector>
using namespace std;

struct Location {
  int xpos;
  int ypos;
  int direction; // N=0, E=1, S=2, W=3
};

void stepForward(vector<string>&grid, Location &currNode) {
  int numRows = grid.size();
  int rowLength = grid[0].size();
  
  switch (currNode.direction) {
    case 0: // going North
    if (currNode.ypos == 0) { //need to insert row north to be safe
      string temp(rowLength, '.');
      grid.insert(grid.begin(), temp);
    } else {
      --currNode.ypos;
    }
    break;
    
    case 1: // going East
    if (currNode.xpos+1 == rowLength) { //each row needs to add '.' to end to be safe
      for (int i=0; i<grid.size(); ++i) grid[i].push_back('.');
    }
    ++currNode.xpos;
    break;
    
    case 2: // going South
    if (currNode.ypos+1 == numRows) { //insert row south to be safe
      string temp(rowLength, '.');
      grid.push_back(temp);
    }
    ++currNode.ypos;
    break;
    
    case 3: // going West
    if (currNode.xpos == 0) { //each row needs to add '.' at string front to be safe
      for (int i=0; i<grid.size(); ++i) grid[i].insert(grid[i].begin(), '.');
    } else {
      --currNode.xpos;
    }
    break;
  }
}

int main() {
  ifstream inputFile("./input.txt");
  vector<string>grid;
  string currLine;
  int causedInfection = 0;
  
  while (inputFile >> currLine) grid.push_back(currLine);

  int startingLoc = grid.size()/2;
  
  Location currNode = { startingLoc, startingLoc, 0 };
  
  for (int i=0; i<10000; ++i) {
    if (grid[currNode.ypos][currNode.xpos] == '#') {
      currNode.direction = (++currNode.direction + 4) % 4;
      grid[currNode.ypos][currNode.xpos] = '.';
    } else {
      currNode.direction = (--currNode.direction + 4) % 4;
      grid[currNode.ypos][currNode.xpos] = '#';
      ++causedInfection;
    }
    
    stepForward(grid, currNode);
  }
  cout << causedInfection << "\n";
}
