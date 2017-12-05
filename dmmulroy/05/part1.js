const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trim()
  .split('\n')
  .map(Number);

const countSteps = arr => {
  let idx = 0;
  let steps = 0;

  while (idx < arr.length) {
    let currIdx = idx;
    let currVal = arr[idx];
    idx += currVal;
    arr[currIdx]++;
    steps++;
  }

  return steps;
};

console.log('output:', countSteps(sanitizedInput));
