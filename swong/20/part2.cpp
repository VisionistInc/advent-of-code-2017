#include <iostream>
#include <fstream>
#include <sstream>
#include <regex>
#include <climits>
#include <cmath>
#include <vector>
#include <map>
using namespace std;

struct Particle {
  int xpos;
  int ypos;
  int zpos;
  int xvel;
  int yvel;
  int zvel;
  int xacc;
  int yacc;
  int zacc;
};

// check if any positions match, if they do, delete them
void removeDuplicates (map<int, Particle>&particles) {
  map<string, vector<int> > checkDups;
  // push everything into a temp map
  for (map<int, Particle>::iterator it=particles.begin(); it!=particles.end(); ++it) {
  // for (int iter=0; iter<particles.size(); ++iter) {
    int iter = it->first;
    Particle tempParticle = particles[iter];
    vector<int>tempVector{iter};
    string key = to_string(tempParticle.xpos) + "::" + to_string(tempParticle.ypos) + "::" + to_string(tempParticle.zpos);
    
    pair<string, vector<int>> temp(key, tempVector);
    auto insertResult = checkDups.insert(temp);
    if (insertResult.second == 0) checkDups[key].push_back(iter);
  }
  // each key that had multiple values in their array had a collision
  for (map<string, vector<int>>::iterator it=checkDups.begin(); it!=checkDups.end(); ++it) {
    if (it->second.size() > 1) {
      for (int dups: it->second) particles.erase(dups);
    }
  }
}

void updateParticles (map<int, Particle>&particles) {
  for (map<int, Particle>::iterator it=particles.begin(); it!=particles.end(); ++it) {
    Particle &currParticle = it->second;
    
    currParticle.xvel += currParticle.xacc;
    currParticle.yvel += currParticle.yacc;
    currParticle.zvel += currParticle.zacc;
    
    currParticle.xpos += currParticle.xvel;
    currParticle.ypos += currParticle.yvel;
    currParticle.zpos += currParticle.zvel;
  }
}

int main() {
  ifstream inputFile("./input.txt");
  string currLine;
  string currVal;
  regex pattern("p=<(.*)>, v=<(.*)>, a=<(.*)>");
  smatch match;
  map<int, Particle>particles;
  int particleNum = 0;
  
  // create the map of particles
  while (getline(inputFile, currLine)) {
    regex_search(currLine, match, pattern);
    Particle tempParticle;
    
    for (int property=1; property<=3; ++property) {
      istringstream ss(match[property]);
      int coord = 0;
      
      switch (property) {
        case 1: 
        while (getline(ss, currVal, ',')) {
          switch (coord) {
            case 0: tempParticle.xpos = stoi(currVal); break;
            case 1: tempParticle.ypos = stoi(currVal); break;
            case 2: tempParticle.zpos = stoi(currVal); break;
          }
          ++coord;
        } break;
        case 2:
        while (getline(ss, currVal, ',')) {
          switch (coord) {
            case 0: tempParticle.xvel = stoi(currVal); break;
            case 1: tempParticle.yvel = stoi(currVal); break;
            case 2: tempParticle.zvel = stoi(currVal); break;
          }
          ++coord;
        } break;
        case 3:
        while (getline(ss, currVal, ',')) {
          switch (coord) {
            case 0: tempParticle.xacc = stoi(currVal); break;
            case 1: tempParticle.yacc = stoi(currVal); break;
            case 2: tempParticle.zacc = stoi(currVal); break;
          }
          ++coord;
        } break;
      }
    }
    pair<int, Particle> temp(particleNum, tempParticle);
    particles.insert(temp);
    ++particleNum;
  }
  
  for (int i=0; i<10000; ++i) {
    removeDuplicates(particles);
    updateParticles(particles);
  }
  
  cout << particles.size() << "\n";
}
