const knotHash = require('../10/part2');

const input = 'jxqlasbh';

const createList = (size = 256) => {
  let list = [];
  for (let idx = 0; idx < size; idx++) {
    list[idx] = idx;
  }
  return list;
};

const zeropad = str =>
  str.length === 4 ? str : `${new Array(4 - str.length + 1).join('0')}${str}`;

const createKnotHashList = (list, input) =>
  list.map(idx => knotHash(`${input}-${idx}`));

const convertKnotHashToBinary = knotHash =>
  knotHash
    .split('')
    .map(hex => zeropad(parseInt(hex, 16).toString(2)))
    .join('');

convertBinaryListToMatrix = binaryList =>
  binaryList.map(row => row.split('').map(Number));

const countUsedSquares = matrix =>
  matrix.reduce((used, row) => {
    return (used += row.reduce((sum, byte) => (sum += byte), 0));
  }, 0);

console.log(
  'ouput:',
  countUsedSquares(
    convertBinaryListToMatrix(
      createKnotHashList(createList(128), input).map(convertKnotHashToBinary)
    )
  )
);
