const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trim()
  .split('\n')
  .map(row => row.split(''));

const calcRowGarbage = row => {
  let garbageCount = 0;
  let stack = [];
  let ignoreNext = false;
  let isGarbage = false;

  row.forEach(char => {
    if (ignoreNext) {
      ignoreNext = !ignoreNext;
      return;
    }

    switch (char) {
      case '{': {
        if (!isGarbage) {
          stack.push(char);
        } else {
          garbageCount++;
        }

        break;
      }
      case '}': {
        if (!isGarbage && stack.length > 0) {
          stack.pop();
        } else if (isGarbage) {
          garbageCount++;
        }
        break;
      }
      case '<': {
        if (isGarbage) garbageCount++;
        isGarbage = true;
        break;
      }
      case '>': {
        isGarbage = false;
        break;
      }
      case '!': {
        ignoreNext = true;
        break;
      }
      default: {
        if (isGarbage) garbageCount++;
        break;
      }
    }
  });
  return garbageCount;
};

console.log(
  'output:',
  sanitizedInput.reduce((prev, curr) => prev + calcRowGarbage(curr), 0)
);
