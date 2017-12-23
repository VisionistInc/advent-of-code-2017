// this problem was killing me, had to adapt this from a guy on reddit
#include <iostream>
#include <fstream>
#include <map>
#include <vector>
#include <math.h>
#include <regex>
using namespace std;

vector<vector<string>>split(const int &step, const vector<string> &state) {
  vector<vector<string>> result;
  for(int x=0; x<state.size(); x+=step) {
      vector<string> row;
      for(int y=0; y<state.size(); y+=step) {
          string temp;
          for(int dy=0; dy<step; ++dy) {
              if(dy!=0) {
                temp+='/';
              }
              temp+=state.at(y+dy).substr(x,step);
            }
          row.push_back(temp);
        }
      result.push_back(row);
    }
  return result;
}

string rotate(string &rule) {
  string rotated = rule;
  
  if (rule.size() <=5) { //2x2
    rotated[0]=rule[1];
    rotated[1]=rule[4];
    rotated[3]=rule[0];
    rotated[4]=rule[3];
  } else { //3x3
    rotated[0]=rule[2];
    rotated[1]=rule[6];
    rotated[2]=rule[10];
    
    rotated[4]=rule[1];
    rotated[6]=rule[9];
    
    rotated[8]=rule[0];
    rotated[9]=rule[4];
    rotated[10]=rule[8];
  }
  rule = rotated;
  return rotated;
}

string flip(const string rule) {
  string flipped = rule;
  if(rule.size() <= 5) {
    swap(flipped[0],flipped[3]);
    swap(flipped[1],flipped[4]);
  }
  else {
    swap(flipped[0],flipped[8]);
    swap(flipped[1],flipped[9]);
    swap(flipped[2],flipped[10]);
  }
  return flipped;
}

int main() {
  ifstream inputFile("./input.txt");
  map<string, string>rules;
  
  string inputRule;
  string resultRule;
  string temp;
  inputFile >> inputRule;
  
  while (inputFile) {
    // strip out the => symbol
    inputFile >> temp >> resultRule;
    string flipped_input(flip(inputRule));
    
    if(rules.find(inputRule)==rules.end()) {
      rules.insert(make_pair(inputRule, resultRule));
    }
    if(rules.find(flipped_input)==rules.end()) {
      rules.insert(make_pair(flipped_input, resultRule));
    }
    
    for (int i=0; i<3; ++i) {
      if(rules.find(rotate(inputRule))==rules.end()) {
        rules.insert(make_pair(inputRule, resultRule));
      }
      if(rules.find(rotate(flipped_input))==rules.end()) {
        rules.insert(make_pair(flipped_input, resultRule));
      }
    }
    inputFile >> inputRule;
  }
  
  vector<string>state ({".#.","..#","###"});
  
  for(int count=0; count<5; ++count) {
    int step = (state.size()%2==0) ? 2 : 3;
    auto blocks=split(step,state);
    state.clear();
    for(auto &row: blocks) {
      for(auto &block: row) block=rules[block];
    }

    state.resize(blocks.size()*(step+1));
    for(auto &c:state) c.resize(blocks.size()*(step+1));

    for(int x=0; x<blocks.size(); ++x)
      for(int y=0; y<blocks[x].size(); ++y)
        for(int dx=0; dx<(step+1); ++dx)
          for(int dy=0; dy<(step+1); ++dy) {
            state.at((step+1)*y+dy).at((step+1)*x+dx) = blocks.at(x).at(y).at(dx+(step+2)*dy);
          }
    }
  
  int totalOn = 0;
  for (int i=0; i<state.size(); ++i) {
    for (char value: state[i]) {
      if (value == '#') ++totalOn;
    }
  }
  
  cout << totalOn << "\n";
}
