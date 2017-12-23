#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>
#include <map>
#include <queue>
using namespace std;

bool execute(vector<string>instructions, map<long, long>&registers, int &nextInstruction, int &mulCount)
{
  if (nextInstruction >= instructions.size() || nextInstruction < 0) {
    return false;
  };

  string command, reg_id, operand_id;
  istringstream instruction(instructions[nextInstruction]);
  long long regA;
  long long regB;

  instruction >> command >> reg_id;
  instruction >> operand_id;

  if (command == "jnz")
    regA = (reg_id[0] >= 'a' && reg_id[0] <= 'z') ? registers[reg_id[0] - 'a'] : stoi(reg_id);
  else
    regA = reg_id[0] - 'a';

  regB = (operand_id[0] >= 'a' && operand_id[0] <= 'z') ? registers[operand_id[0] - 'a'] : stoi(operand_id);

  if (command == "set") registers[regA] = regB;
  else if (command == "sub") registers[regA] -= regB;
  else if (command == "mul") {
    registers[regA] *= regB;
    ++mulCount;
  }
  else if (command == "jnz" && regA != 0) {
    nextInstruction += regB;
    return true;
  }

  ++nextInstruction;
  return true;
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  vector<string>instructions;
  map<long, long>registers;
  int nextInstruction = 0;
  bool continueExecute = true;
  int mulCount = 0;

  // create the instructions
  while (getline(inputFile, currLine)) {
    instructions.push_back(currLine);
  }

  while (continueExecute) {
    continueExecute = execute(instructions, registers, nextInstruction, mulCount);
  }
  cout << mulCount << "\n";
}
