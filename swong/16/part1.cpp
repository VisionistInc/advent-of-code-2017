#include <iostream>
#include <fstream>
#include <sstream>
#include <regex>
#include <string>
#include <vector>
using namespace std;

void reverse(char (&programs)[16], int amount) {
  vector<char>temp;

  // copy the elements to be shifted to the end
  for (int i=0; i<16-amount; ++i) temp.push_back(programs[i]);
  // shift the elements from the back to the front
  for (int i=0; i<amount; ++i) programs[i] = programs[16-amount+i];
  // copy the temp values back into the array
  for (int i=0; i<temp.size(); ++i) programs[i+amount] = temp[i];
}

void swap(char (&programs)[16], int A, int B) {
  char temp = programs[A];
  programs[A] = programs[B];
  programs[B] = temp;
}

int findIndex(char (&programs)[16], char seek) {
  for (int i=0; i<16; ++i) {
    if (programs[i] == seek) return i;
  }
}

void swap(char (&programs)[16], string A, string B) {
  char swapA = A[0];
  char swapB = B[0];

  int indexA = findIndex(programs, swapA);
  int indexB = findIndex(programs, swapB);

  swap(programs, indexA, indexB);
}

void executeMove(char (&programs)[16], smatch match) {
  if (match[1] == 's') {
    reverse(programs, stoi(match[2]));
  } else if (match[1] == 'x') {
    swap(programs, stoi(match[2]), stoi(match[3]));
  } else if (match[1] == 'p') {
    swap(programs, match[2], match[3]);
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string danceMove;
  regex pattern("([a-z])([a-z0-9]+)\\/?(.*)");
  smatch match;

  // create the initial array
  char programs[16] = { 'a', 'b', 'c', 'd', 'e', 'f', 'g',
  'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p' };

  while (getline(inputFile, danceMove, ',')) {
    bool result = regex_search(danceMove, match, pattern);
    if (result) {
      executeMove(programs, match);
    }
  }

  for (char program: programs) cout << program << "";
  cout << "\n";
}
