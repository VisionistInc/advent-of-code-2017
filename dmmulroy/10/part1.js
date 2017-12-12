const lengths = require('./input');

const createList = () => {
  let list = [];
  for (let idx = 0; idx < 256; idx++) {
    list[idx] = idx;
  }
  return list;
};

const knotHash = (list, lengths) => {
  let skip = 0;
  let currentPos = 0;

  lengths.forEach(length => {
    let sliceToReverse = [];

    for (let idx = 0; idx < length; idx++) {
      sliceToReverse[idx] = list[(currentPos + idx) % list.length];
    }

    for (let idx = 0; idx < length; idx++) {
      list[(currentPos + idx) % list.length] = sliceToReverse[length - idx - 1];
    }
    currentPos += length + skip;
    skip++;
  });

  return list;
};

const knotHashList = knotHash(createList(), lengths);
console.log(knotHashList.length);

console.log('output:', knotHashList[0] * knotHashList[1]);
