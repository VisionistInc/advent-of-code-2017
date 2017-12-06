const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trimRight()
  .split('\n')
  .map(row => row.split(/\s+/).map(Number));

const calcRowSum = row => {
  let valOne = 0;
  let valTwo = 0;

  for (let i = 0; i < row.length; i++) {
    if (valOne > 0) break;
    for (let j = i + 1; j < row.length; j++) {
      if (row[i] % row[j] === 0) {
        valOne = row[i];
        valTwo = row[j];
        break;
      }
      if (row[j] % row[i] === 0) {
        valOne = row[j];
        valTwo = row[i];
        break;
      }
    }
  }

  return valOne / valTwo;
};

const checksum = spreadsheet =>
  spreadsheet.reduce((sum, row) => sum + calcRowSum(row), 0);

console.log('output:', checksum(sanitizedInput));
