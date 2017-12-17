#include <iostream>
#include <fstream>
#include <sstream>
#include <regex>
#include <string>
#include <vector>
using namespace std;

void reverse(string &programs, int amount) {
  vector<char>temp;

  // copy the elements to be shifted to the end
  for (int i=0; i<16-amount; ++i) temp.push_back(programs[i]);
  // shift the elements from the back to the front
  for (int i=0; i<amount; ++i) programs[i] = programs[16-amount+i];
  // copy the temp values back into the array
  for (int i=0; i<temp.size(); ++i) programs[i+amount] = temp[i];
}

void swap(string &programs, int A, int B) {
  char temp = programs[A];
  programs[A] = programs[B];
  programs[B] = temp;
}

int findIndex(string &programs, char seek) {
  for (int i=0; i<16; ++i) {
    if (programs[i] == seek) return i;
  }
}

void swap(string &programs, string A, string B) {
  char swapA = A[0];
  char swapB = B[0];

  int indexA = findIndex(programs, swapA);
  int indexB = findIndex(programs, swapB);

  swap(programs, indexA, indexB);
}

void executeMove(string &programs, vector<string> match) {
  if (match[1] == "s") {
    reverse(programs, stoi(match[2]));
  } else if (match[1] == "x") {
    swap(programs, stoi(match[2]), stoi(match[3]));
  } else if (match[1] == "p") {
    swap(programs, match[2], match[3]);
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string danceMove;
  regex pattern("([a-z])([a-z0-9]+)\\/?(.*)");
  vector<vector<string>>danceMoves;
  smatch match;

  // create the initial array
  string programs = "abcdefghijklmnop";

  while (getline(inputFile, danceMove, ',')) {
    regex_search(danceMove, match, pattern);
    vector<string>temp;
    temp.push_back(match[0]);
    temp.push_back(match[1]);
    temp.push_back(match[2]);
    temp.push_back(match[3]);

    danceMoves.push_back(temp);
  }

  for (vector<string> move: danceMoves) {
    executeMove(programs, move);
  }

  cout << programs << "\n";
}
