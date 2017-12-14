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

const createKnotHashList = (list, input) => {
  const _list = createList();
  return list.map(idx => knotHash(`${input}-${idx}`));
};

const convertKnotHashToBinary = knotHash =>
  knotHash
    .split('')
    .map(hex => zeropad(parseInt(hex, 16).toString(2)))
    .join('');

convertBinaryListToMatrix = binaryList =>
  binaryList.map(row => row.split('').map(Number));

const matrix = convertBinaryListToMatrix(
  createKnotHashList(createList(128), input).map(convertKnotHashToBinary)
);

const isSafe = (martix, row, column, visited) =>
  row >= 0 &&
  row < 128 &&
  column >= 0 &&
  column < 128 &&
  (matrix[row][column] === 1 && !visited[row][column]);

const dfs = (matrix, row, column, visited) => {
  // up down left right
  const rowNeighbors = [1, -1, 0, 0];
  const columnNeighbors = [0, 0, -1, 1];

  visited[row][column] = true;

  for (let neighbor = 0; neighbor < 4; neighbor++) {
    if (
      isSafe(
        matrix,
        row + rowNeighbors[neighbor],
        column + columnNeighbors[neighbor],
        visited
      )
    ) {
      dfs(
        matrix,
        row + rowNeighbors[neighbor],
        column + columnNeighbors[neighbor],
        visited
      );
    }
  }
};

const countRegions = matrix => {
  let visited = createList(128).map(x => createList(128).map(y => false));
  let count = 0;

  for (let row = 0; row < 128; row++) {
    for (let column = 0; column < 128; column++) {
      if (matrix[row][column] === 1 && !visited[row][column]) {
        dfs(matrix, row, column, visited);
        count++;
      }
    }
  }
  return count;
};

console.log('countRegions', countRegions(matrix));
