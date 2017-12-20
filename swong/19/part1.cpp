#include <iostream>
#include <fstream>
#include <vector>
#include <ctype.h>
using namespace std;

bool traverseMaze (vector<string>maze, int &x_loc, int &y_loc, char &direction, vector<char>&pathTaken, int maxX, int maxY) {
  char currVal = maze[y_loc][x_loc];
  
  if (currVal == ' ') return false;
  else if (currVal == '|') {
    switch (direction) {
      case 'S': ++y_loc; break;
      case 'N': --y_loc; break;
      case 'E': ++x_loc; break;
      case 'W': --x_loc; break;
    }
  } else if (isalpha(currVal)) {
    pathTaken.push_back(currVal);
    switch (direction) {
      case 'S': ++y_loc; break;
      case 'N': --y_loc; break;
      case 'E': ++x_loc; break;
      case 'W': --x_loc; break;
    }
  } else if (currVal == '-') {
    switch (direction) {
      case 'E': ++x_loc; break;
      case 'W': --x_loc; break;
      case 'N': --y_loc; break;
      case 'S': ++y_loc; break;
    }
  } else if (currVal == '+') {
    if (direction == 'S' || direction == 'N') {
      if (x_loc-1 < 0) {
        ++x_loc;
        direction = 'E';
      } else if (x_loc+1 > maxX) {
        --x_loc;
        direction = 'W';
      } else if (maze[y_loc][x_loc-1] != ' ') {
        --x_loc;
        direction = 'W';
      } else {
        ++x_loc;
        direction = 'E';
      }
    } else if (direction == 'E' || direction == 'W') {
      if (y_loc-1 < 0) {
        ++y_loc;
        direction = 'S';
      } else if (y_loc+1 > maxY) {
        --y_loc;
        direction = 'N';
      } else if (maze[y_loc-1][x_loc] != ' ') {
        --y_loc;
        direction = 'N';
      } else {
        ++y_loc;
        direction = 'S';
      }
    }
  }
  return true;
}

int main() {
  ifstream inputFile("./input.txt");
  vector<string>maze;
  vector<char>pathTaken;
  string currLine;
  int x_loc = 0;
  int y_loc = 0;
  char direction = 'S';
  bool traverse = true;

  while (getline(inputFile, currLine)) maze.push_back(currLine);

  // find the entrance
  for (int i=0; i<maze[0].size(); ++i) {
    if (maze[0][i] == '|') {
      x_loc = i;
      break;
    }
  }

  int maxX = maze[0].size();
  int maxY = maze.size();

  while (traverse && x_loc >= 0 && y_loc >= 0 && x_loc < maxX && y_loc < maxY) {
    traverse = traverseMaze(maze, x_loc, y_loc, direction, pathTaken, maxX, maxY);
  }

  for (int i=0; i<pathTaken.size(); ++i) cout << pathTaken[i];
  cout << "\n";
}
