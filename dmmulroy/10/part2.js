const lengths = '120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113';

const createList = () => {
  let list = [];
  for (let idx = 0; idx < 256; idx++) {
    list[idx] = idx;
  }
  return list;
};

const convertToASCII = str => str.split('').map(char => char.charCodeAt());

const knotHash = (list, lengths) => {
  let skip = 0;
  let currentPos = 0;
  let denseHash = [];

  lengths.push(...[17, 31, 73, 47, 23]);

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
    let XOR = 0;
    for (let blockValue = 0; blockValue < 16; blockValue++) {
      XOR = XOR ^ list[block * 16 + blockValue];
    }
    denseHash[block] = XOR;
  }

  return denseHash
    .map(byte => {
      const hex = byte.toString(16);
      return hex.length === 2 ? hex : `0${hex}`;
    })
    .join('');
};

console.log(knotHash(createList(), convertToASCII(lengths)));
