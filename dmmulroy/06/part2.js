const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trim()
  .split(/\s+/)
  .map(Number);

const countRedistributionCycles = arr => {
  let currentMaxIdx = 0;
  let currentMaxAmt = 0;
  let previousMemoryBanks = new Set();
  let redistributionCycles = 0;

  while (!previousMemoryBanks.has(arr.toString())) {
    // Add the current memoryBanks as a string to the set
    previousMemoryBanks.add(arr.toString());

    // Find the new memory bank to redistribute
    arr.forEach((num, idx) => {
      if (num > currentMaxAmt) {
        currentMaxAmt = num;
        currentMaxIdx = idx;
      }
    });

    // Set the new memory bank to redistribute to 0
    arr[currentMaxIdx] = 0;

    // Redistribute memoryBanks
    for (let i = currentMaxIdx + 1; currentMaxAmt !== 0; i++) {
      arr[i % arr.length]++;
      currentMaxAmt--;
    }

    // Increment redistributionCycles
    redistributionCycles++;

    // Reset tracking vars
    currentMaxAmt = 0;
    currentMaxIdx = 0;
  }
  return [redistributionCycles, arr];
};

console.log(
  'output:',
  countRedistributionCycles(countRedistributionCycles(sanitizedInput)[1])[0]
);
