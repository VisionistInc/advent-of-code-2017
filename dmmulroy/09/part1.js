const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trim()
  .split('\n')
  .map(row => row.split(''));

const calcRowScore = row => {
  let score = 0;
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
        if (!isGarbage) stack.push(char);
        break;
      }
      case '}': {
        if (!isGarbage && stack.length > 0) {
          score += stack.length;
          stack.pop();
        }
        break;
      }
      case '<': {
        isGarbage = true;
        break;
      }
      case '>': {
        isGarbage = false;
        break;
      }
      case '!': {
        ignoreNext = true;
      }
      default:
        break;
    }
  });
  return score;
};

console.log(
  'output:',
  sanitizedInput.reduce((prev, curr) => prev + calcRowScore(curr), 0)
);
