const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trim()
  .split('\n')
  .map(row => {
    const [depth, range] = row.split(': ');
    return [
      Number(depth),
      { range: Number(range), scanPosition: 0, scanDirection: '+' }
    ];
  });

const createFirewall = input =>
  input.reduce((prev, curr) => {
    const [depth, range] = curr;
    prev[depth] = range;
    return prev;
  }, []);

const calcNewScanIdxAndDirection = (range, currentScanIdx, scanDirection) => {
  if (scanDirection === '+') {
    if (currentScanIdx + 1 === range) {
      return [currentScanIdx - 1, '-'];
    } else {
      return [currentScanIdx + 1, '+'];
    }
  } else {
    if (currentScanIdx === 0) {
      return [currentScanIdx + 1, '+'];
    } else {
      return [currentScanIdx - 1, '-'];
    }
  }
};

const scanFirewall = firewall =>
  firewall.map(({ range, scanPosition, scanDirection }) => {
    const [newScanIdx, newScanDirection] = calcNewScanIdxAndDirection(
      range,
      scanPosition,
      scanDirection
    );

    return { range, scanPosition: newScanIdx, scanDirection: newScanDirection };
  });

const calcSafeDelay = firewall => {
  let delay = 0;
  let safe = false;

  while (!safe) {
    console.log('delay', delay);
    for (let idx = 0; idx < firewall.length; idx++) {
      safe = true;
      if (
        firewall[idx] &&
        (delay + idx) % ((firewall[idx].range - 1) * 2) === 0
      ) {
        safe = false;
        break;
      }
    }

    delay++;
  }

  return delay;
};
// console.log('firewall', createFirewall(sanitizedInput).length);
console.log('output:', calcSafeDelay(createFirewall(sanitizedInput)));
