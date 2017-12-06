#include <iostream>
#include <fstream>
#include <sstream>
#include <unordered_map>
using namespace std;

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  int validPassphrases = 0;

  // read the file line by line
  while (getline(inputFile, currLine)) {
    unordered_map<string, int> map;
    istringstream is(currLine);
    string word;
    bool validPassphrase = true;

    while (is >> word) {
      pair<string, int> temp (word, 1);
      auto result = map.insert(temp);

      // if unable to insert, key/word already exists, invalid
      if (result.second == 0) {
        validPassphrase = false;
        break;
      }
    }
    if (validPassphrase == true) validPassphrases++;
  }
  cout << validPassphrases << "\n";
}
