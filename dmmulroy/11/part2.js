const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input.trim().split(',');

// thank god for https://www.redblobgames.com/grids/hexagons/#neighbors

const calcDistance = ({ x, y, z }) =>
  (Math.abs(x - 0) + Math.abs(y - 0) + Math.abs(z - 0)) / 2;

const calcMaxDistance = steps =>
  steps.reduce(
    (prev, curr, idx) => {
      switch (curr) {
        case 'n':
          prev.y++;
          prev.z--;
          break;
        case 's':
          prev.y--;
          prev.z++;
          break;
        case 'nw':
          prev.x--;
          prev.y++;
          break;
        case 'se':
          prev.x++;
          prev.y--;
          break;
        case 'ne':
          prev.x++;
          prev.z--;
          break;
        case 'sw':
          prev.x--;
          prev.z++;
          break;
        default:
          throw new Error('Something went wrong');
          break;
      }

      prev.max = Math.max(prev.max, calcDistance(prev));

      return idx === steps.length - 1 ? prev.max : prev;
    },
    { x: 0, y: 0, z: 0, max: 0 }
  );

console.log('output:', calcMaxDistance(sanitizedInput));
