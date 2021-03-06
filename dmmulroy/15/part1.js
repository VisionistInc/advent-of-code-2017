const divisor = 2147483647;
const aFactor = 16807;
const bFactor = 48271;

const judge = (a, b) => a.slice(-16) === b.slice(-16);

const zeropad = str =>
  str.length >= 16 ? str : `${new Array(17 - str.length).join('0')}${str}`;

const toBinary = str => parseInt(str, 10).toString(2);

const calculatePairs = (a = 289, b = 629) => {
  let pairs = 0;
  for (let iter = 0; iter < 40000000; iter++) {
    a = (a * aFactor) % divisor;
    b = (b * bFactor) % divisor;

    if (judge(zeropad(toBinary(a)), zeropad(toBinary(b)))) pairs++;
  }
  return pairs;
};

// console.time('time');
console.log('output:', calculatePairs());
// console.timeEnd('time');
