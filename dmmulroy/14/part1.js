const knotHash = require('../10/part2');

const input = 'jxqlasbh';

const createList = () => {
  let list = [];
  for (let idx = 0; idx < 128; idx++) {
    list[idx] = idx;
  }
  return list;
};

const createKnotHashList = (list, input) =>
  list.map(idx => knotHash(list, `${input}-${idx}`));

const convertKnotHashToBinary = knotHash =>
  knotHash
    .match(/.{1,2}/)
    .map(hex => parseInt(hex, 16).toString(2))
    .join('');

const countUsedSquares = matrix => {};

console.log(
  'knotHashList',
  createKnotHashList(createList(), input).map(convertKnotHashToBinary)
);
