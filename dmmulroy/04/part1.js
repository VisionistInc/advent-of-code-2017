const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const result = input
  .trim()
  .split('\n')
  .map(row =>
    row.split(' ').reduce((prev, curr, idx, arr) => {
      if (prev[curr]) {
        prev[curr] += 1;
      } else {
        prev[curr] = 1;
      }
      if (idx === arr.length - 1)
        return Object.values(prev).filter(x => x > 1).length > 0;
      return prev;
    }, {})
  )
  .filter(val => !val).length;

console.log('result', result);
