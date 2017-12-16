const fs = require('fs');
const path = require('path');

const memoize = require('lodash.memoize');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const danceMoves = input.split(',');

let letters = [
  'a',
  'b',
  'c',
  'd',
  'e',
  'f',
  'g',
  'h',
  'i',
  'j',
  'k',
  'l',
  'm',
  'n',
  'o',
  'p'
];

const swap = (arr, idxA, idxB) => {
  const tmp = arr[idxA];
  arr[idxA] = arr[idxB];
  arr[idxB] = tmp;
};

const spin = (letters, times) => [
  ...letters.slice(times * -1),
  ...letters.slice(0, letters.length - times)
];

const exchange = (letters, idxA, idxB) => {
  const updatedLetters = [...letters];
  swap(updatedLetters, idxA, idxB);
  return updatedLetters;
};

const partner = (letters, a, b) => {
  const updatedLetters = [...letters];
  const indexOfA = updatedLetters.indexOf(a);
  const indexOfB = updatedLetters.indexOf(b);
  swap(updatedLetters, indexOfA, indexOfB);
  return updatedLetters;
};

const performDanceMoves = (letters, danceMoves) => {
  danceMoves.forEach(danceMove => {
    switch (danceMove[0]) {
      case 's':
        const times = danceMove.slice(1);
        letters = spin(letters, Number(times));
        break;
      case 'x':
        const [idxA, idxB] = danceMove.slice(1).split('/');
        letters = exchange(letters, Number(idxA), Number(idxB));
        break;
      case 'p':
        const [a, b] = danceMove.slice(1).split('/');
        letters = partner(letters, a, b);
        break;
      default:
        throw new Error('Something went wrong!');
        break;
    }
  });

  return letters.join('');
};

const memoizedPerformDanceMoves = memoize(performDanceMoves);

for (let iter = 0; iter < 1000000000; iter++) {
  letters = memoizedPerformDanceMoves(letters, danceMoves);
}

console.log('output:', letters);
