#include <iostream>
#include <fstream>
#include <map>
#include <regex>
#include <string.h>
using namespace std;

// the possible operators are:
// >, <, >=, <=, ==, !=
bool checkCondition(string condReg, string op, string condValue, map<string, int> &map) {
  int compValue = stoi(condValue);

  if (op == ">") return map[condReg] > compValue;
  else if (op == "<") return map[condReg] < compValue;
  else if (op == ">=") return map[condReg] >= compValue;
  else if (op == "<=") return map[condReg] <= compValue;
  else if (op == "==") return map[condReg] == compValue;
  else if (op == "!=") return map[condReg] != compValue;
  else return false;
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  regex pattern("(\\w+) (\\w+) (-?\\w+) if (\\w+) (.*) (-?\\d+)");
  smatch match;
  int highestRegisterValue = 0;

  map<string, int>registerMap;

  while (getline(inputFile, currLine)) {
    bool result = regex_search(currLine, match, pattern);

    // match[] = [currLine, affectedRegister, incOrDec, howMuch, condReg, operation, condValue]
    if (result) {
      bool condition = checkCondition(match[4], match[5], match[6], registerMap);

      // initialize both registers in map to 0 if they don't already exist
      pair<string, int> affectedRegister(match[1], 0);
      registerMap.insert(affectedRegister);

      pair<string, int> conditionRegister(match[4], 0);
      registerMap.insert(affectedRegister);

      if (condition) {
        if (match[2] == "inc") registerMap[match[1]] += stoi(match[3]);
        else if (match[2] == "dec") registerMap[match[1]] -= stoi(match[3]);

        // it's possible the highest value occurs during a dec since you can subtract a negative value
        if (registerMap[match[1]] > highestRegisterValue) highestRegisterValue = registerMap[match[1]];
      }
    }
  }

  cout << highestRegisterValue << "\n";
}
