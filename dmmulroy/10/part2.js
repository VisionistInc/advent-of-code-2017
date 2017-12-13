const lengths = require('./input');

const createList = () => {
  let list = [];
  for (let idx = 0; idx < 256; idx++) {
    list[idx] = idx.toString().charCodeAt();
  }
  return list.concat([17, 31, 73, 47, 23]);
};

const knotHash = (list, lengths) => {
  let skip = 0;
  let currentPos = 0;
  let denseHash = [];

  for (let rounds = 0; rounds < 64; rounds++) {
    lengths.forEach(length => {
      let sliceToReverse = [];

      for (let idx = 0; idx < length; idx++) {
        sliceToReverse[idx] = list[(currentPos + idx) % list.length];
      }

      for (let idx = 0; idx < length; idx++) {
        list[(currentPos + idx) % list.length] =
          sliceToReverse[length - idx - 1];
      }
      currentPos += (length + skip) % list.length;
      skip++;
    });
  }

  for (let block = 0; block < 16; block++) {
    for (let blockValue = 0; blockValue < 16; blockValue++) {
      denseHash[block] = denseHash[block] || 0 ^ list[block * 16 + blockValue];
    }
  }

  return denseHash.map(byte => ('0' + (byte & 0xff)).toString(16)).join('');
};

console.log(knotHash(createList(), lengths));
