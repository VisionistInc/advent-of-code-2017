#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>
#include <map>
#include <queue>
using namespace std;

bool parseInstruction(
  vector<string>instructions,
  map<long, long>&registers,
  int &numSent,
  int &nextInstruction,
  queue<long long>&queueRcv,
  queue<long long>&queueSnd
)
{
  if (nextInstruction >= instructions.size() || nextInstruction < 0) {
    return true;
  };

  string command, reg_id, operand_id;
  istringstream instruction(instructions[nextInstruction]);
  long long regA;
  long long regB;

  instruction >> command >> reg_id;

  if (command == "snd" || command == "rcv")
    operand_id = reg_id;
  else
    instruction >> operand_id;

  if (command == "jgz")
    regA = (reg_id[0] >= 'a' && reg_id[0] <= 'z') ? registers[reg_id[0] - 'a'] : stoi(reg_id);
  else
    regA = reg_id[0] - 'a';

  regB = (operand_id[0] >= 'a' && operand_id[0] <= 'z') ? registers[operand_id[0] - 'a'] : stoi(operand_id);

  if (command == "snd") {
    queueSnd.push(regB);
    ++numSent;
  }
  else if (command == "set") registers[regA] = regB;
  else if (command == "add") registers[regA] += regB;
  else if (command == "mul") registers[regA] *= regB;
  else if (command == "mod") registers[regA] %= regB;
  else if (command == "rcv") {
    if (queueRcv.empty()) {
      return true;
    } else {
      registers[regA] = queueRcv.front();
      queueRcv.pop();
    }
  }
  else if (command == "jgz" && regA > 0) {
    nextInstruction += regB;
    return false;
  }

  ++nextInstruction;
  return false;
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  vector<string>instructions;
  map<long, long>registersProgA;
  map<long, long>registersProgB;
  int nextInstructionA = 0;
  int nextInstructionB = 0;
  int numSentA = 0;
  int numSentB = 0;
  queue<long long> queueA;
  queue<long long> queueB;

  // create the instructions
  while (getline(inputFile, currLine)) {
    instructions.push_back(currLine);
  }

  registersProgA['p' - 'a'] = 0;
  registersProgB['p' - 'a'] = 1;

  while (true) {
    bool stopA = parseInstruction(instructions, registersProgA, numSentA, nextInstructionA, queueA, queueB);
    bool stopB = parseInstruction(instructions, registersProgB, numSentB, nextInstructionB, queueB, queueA);
    if (stopA && stopB) break;
  }

  cout << numSentB << "\n";
}
