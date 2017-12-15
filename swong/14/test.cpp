#include <iostream>
using namespace std;

int main() {
  string input = "oundnydw";
  string newString = input + '-' + to_string(0);

  cout << newString << "\n";
}
