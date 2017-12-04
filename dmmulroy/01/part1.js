const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');
const sanitizedInput = input
  .trim()
  .split('')
  .map(Number);

const inverseCaptcha = arr =>
  arr.reduce(
    (prev, curr, idx) =>
      arr[(idx + 1) % arr.length] === curr ? prev + curr : prev,
    0
  );

console.log('output:', inverseCaptcha(sanitizedInput));
