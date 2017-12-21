#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <map>
using namespace std;

void executeInstruction(map<string, long int>&registers,
  string command,
  string reg,
  int instruction2,
  int &lastSoundPlayed,
  int &nextInstruction,
  bool &stop)
{
  if (command == "snd") lastSoundPlayed = registers[reg];
  else if (command == "set") registers[reg] = instruction2;
  else if (command == "add") registers[reg] += instruction2;
  else if (command == "mul") registers[reg] *= instruction2;
  else if (command == "mod") registers[reg] = registers[reg] % instruction2;
  else if (command == "rcv" && registers[reg] != 0) {
    stop = true;
    return;
  }
  else if (command == "jgz" && registers[reg] > 0) {
    nextInstruction += instruction2;
    return;
  }

  ++nextInstruction;
}

bool is_number(const string& s) {
  string::const_iterator it = s.begin();
  while ((it != s.end() && isdigit(*it) )|| *it=='-') ++it;
  return !s.empty() && it == s.end();
}

void parseInstruction(map<string, long int>&registers,
  vector<string>instruction,
  int &lastSoundPlayed,
  int &nextInstruction,
  bool &stop)
{
  string command = instruction[0];
  string reg = instruction[1];

  if (is_number(instruction[2])) {
    int instruction2 = stoi(instruction[2]);
    executeInstruction(registers, command, reg, instruction2, lastSoundPlayed, nextInstruction, stop);
  } else {
    int instruction2 = registers[instruction[2]];
    executeInstruction(registers, command, reg, instruction2, lastSoundPlayed, nextInstruction, stop);
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  string currVal;
  map<string, long int>registers;
  int lastSoundPlayed = 0;
  int nextInstruction = 0;
  vector<vector<string> >instructions;
  bool stop = false;

  // create the instructions
  while (getline(inputFile, currLine)) {
    stringstream ss(currLine);
    vector<string>instruction;

    while (ss >> currVal) {
      instruction.push_back(currVal);
    }

    instructions.push_back(instruction);
  }

  while (nextInstruction >= 0 && nextInstruction < instructions.size() && stop == false) {
    parseInstruction(registers, instructions[nextInstruction], lastSoundPlayed, nextInstruction, stop);
  }

  cout << lastSoundPlayed << "\n";
}
