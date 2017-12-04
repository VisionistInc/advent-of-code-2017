const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trimRight()
  .split('\n')
  .map(row => row.split(/\s+/).map(Number));

const calcRowDifference = row => {
  let min = null;
  let max = 0;

  row.forEach(val => {
    if (val < min || min === null) min = val;
    if (val > max) max = val;
  });

  return max - min;
};

const checksum = spreadsheet =>
  spreadsheet.reduce((sum, row) => sum + calcRowDifference(row), 0);

console.log('output:', checksum(sanitizedInput));
