const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input.trim().split(',');

// thank god for https://www.redblobgames.com/grids/hexagons/#neighbors

const calcShortestPath = steps =>
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

      return idx === steps.length - 1
        ? (Math.abs(prev.x - 0) + Math.abs(prev.y - 0) + Math.abs(prev.z - 0)) /
            2
        : prev;
    },
    { x: 0, y: 0, z: 0 }
  );

console.log('output:', calcShortestPath(sanitizedInput));
