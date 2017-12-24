// apparently this is what the instructions reduce to
// h increments when d*e == b (aka non prime), once per b
// program exits when b == c while b increments by 17 per loop
#include <iostream>
#include <map>
using namespace std;

int main() {
  map<char, int>registers;
  registers['a'] = 1;
  registers['b'] = 67;
  registers['c'] = 67;
  registers['d'] = 0;
  registers['e'] = 0;
  registers['f'] = 0;
  registers['g'] = 0;
  registers['h'] = 0;

  registers['b'] = registers['b'] * 100 + 100000;
  registers['c'] = registers['b'] + 17000;
  do {
    registers['f'] = 1;
    registers['d'] = 2;
    for (int d = registers['d']; d * d < registers['b']; ++d) {
      if (registers['b'] % d == 0) {
        registers['f'] = 0;
        break;
      }
    }
    if (registers['f'] == 0) registers['h']++;
    registers['g'] = registers['b'] - registers['c'];
    registers['b'] += 17;
  } while (registers['g'] != 0);

  cout << registers['h'] << "\n";
}
