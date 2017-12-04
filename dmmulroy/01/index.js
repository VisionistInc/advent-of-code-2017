const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');
const sanitizedInput = input
  .trim()
  .split('')
  .map(Number);

const inverseCaptcha = arr =>
  arr.reduce((prev, curr, idx) => {
    if (idx === 0) return prev;

    if (idx === arr.length - 1) {
      if (arr[idx - 1] === curr) prev += curr;

      return arr[0] == curr ? prev + curr : prev;
    }

    if (arr[idx - 1] === curr) return prev + curr;

    return prev;
  }, 0);

console.log('output:', inverseCaptcha(sanitizedInput));
