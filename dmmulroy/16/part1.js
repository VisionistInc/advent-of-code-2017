const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const danceMoves = input.split(',');

const spin = (map, times) =>
  Object.entries(map).reduce((prev, [key, value], idx, arr) => {
    prev[key] = (map[key] + times) % arr.length;
    return prev;
  }, {});

const exchange = (map, idxA, idxB) => {
  let keyA, keyB;
  Object.entries(map).forEach(([key, value]) => {
    if (value === idxA) keyA = key;
    if (value === idxB) keyB = key;
  });

  return Object.assign({}, map, { [keyA]: map[keyB], [keyB]: map[keyA] });
};

const partner = (map, keyA, keyB) =>
  Object.assign({}, map, { [keyA]: map[keyB], [keyB]: map[keyA] });

const performDanceMoves = danceMoves => {
  let letterIdxMap = {
    a: 0,
    b: 1,
    c: 2,
    d: 3,
    e: 4,
    f: 5,
    g: 6,
    h: 7,
    i: 8,
    j: 9,
    k: 10,
    l: 11,
    m: 12,
    n: 13,
    o: 14,
    p: 15
  };

  danceMoves.forEach(danceMove => {
    switch (danceMove[0]) {
      case 's':
        const times = danceMove.slice(1);
        letterIdxMap = spin(letterIdxMap, Number(times));
        break;
      case 'x':
        const [idxA, idxB] = danceMove.slice(1).split('/');
        letterIdxMap = exchange(letterIdxMap, Number(idxA), Number(idxB));
        break;
      case 'p':
        const [keyA, keyB] = danceMove.slice(1).split('/');
        letterIdxMap = partner(letterIdxMap, keyA, keyB);
        break;
      default:
        throw new Error('Something went wrong!');
        break;
    }
  });

  console.log(letterIdxMap);

  return Object.entries(letterIdxMap)
    .reduce((prev, [key, value]) => {
      prev[value] = key;
      return prev;
    }, [])
    .join('');
};

console.log('output:', performDanceMoves(danceMoves));
