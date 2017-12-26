#include <iostream>
#include <vector>
using namespace std;

void stepRight (vector<int>&tape, int &currIndex) {
  if (currIndex+1 == tape.size()) tape.push_back(0);
  ++currIndex;
}

void stepLeft (vector<int>&tape, int &currIndex) {
  if (currIndex == 0) tape.insert(tape.begin(), 0);
  else --currIndex;
}

int main() {
  int inputSteps = 12523873;
  int iterations = 0;
  vector<int>tape;
  int checksum = 0;
  int currIndex = 0;
  
  // initialize
  char currentState = 'A';
  tape.push_back(0);
  
  while(true) {
    switch (currentState) {
      case 'A':
      if (tape[currIndex] == 0) {
        tape[currIndex] = 1;
        stepRight(tape, currIndex);
        currentState = 'B';
        ++checksum;
      } else {
        tape[currIndex] = 1;
        stepLeft(tape, currIndex);
        currentState = 'E';
      }
      break;
      
      case 'B':
      if (tape[currIndex] == 0) {
        tape[currIndex] = 1;
        stepRight(tape, currIndex);
        currentState = 'C';
        ++checksum;
      } else {
        tape[currIndex] = 1;
        stepRight(tape, currIndex);
        currentState = 'F';
      }
      break;
      
      case 'C':
      if (tape[currIndex] == 0) {
        tape[currIndex] = 1;
        stepLeft(tape, currIndex);
        currentState = 'D';
        ++checksum;
      } else {
        tape[currIndex] = 0;
        stepRight(tape, currIndex);
        currentState = 'B';
        --checksum;
      }
      break;
      
      case 'D':
      if (tape[currIndex] == 0) {
        tape[currIndex] = 1;
        stepRight(tape, currIndex);
        currentState = 'E';
        ++checksum;
      } else {
        tape[currIndex] = 0;
        stepLeft(tape, currIndex);
        currentState = 'C';
        --checksum;
      }
      break;
      
      case 'E':
      if (tape[currIndex] == 0) {
        tape[currIndex] = 1;
        stepLeft(tape, currIndex);
        currentState = 'A';
        ++checksum;
      } else {
        tape[currIndex] = 0;
        stepRight(tape, currIndex);
        currentState = 'D';
        --checksum;
      }
      break;
      
      case 'F':
      if (tape[currIndex] == 0) {
        tape[currIndex] = 1;
        stepRight(tape, currIndex);
        currentState = 'A';
        ++checksum;
      } else {
        tape[currIndex] = 1;
        stepRight(tape, currIndex);
        currentState = 'C';
      }
      break;
    }
    
    ++iterations;
    if (iterations == inputSteps) break;
  }
  cout << checksum << "\n";
}
