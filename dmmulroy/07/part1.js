const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trim()
  .split('\n')
  .reduce((prev, curr) => {
    const { 1: key, 2: weight, 3: childrenString } = curr.includes('->')
      ? curr.match(/(\w+)\s\((\d+)\)\s->\s(.+)/)
      : curr.match(/(\w+)\s\((\d+)\)/);
    const children = childrenString
      ? childrenString.replace(/\s/g, '').split(',')
      : [];
    prev[key] = { weight, children };
    return prev;
  }, {});

const copyChildren = input => child => {
  const { weight, children } = input[child];
  if (children.length > 0) {
    return {
      weight,
      children: children.map(copyChildren(input))
    };
  } else {
    return input[child];
  }
};

const createTreeMap = input => {
  const populatedMap = Object.entries(input).reduce((prev, curr) => {
    const [key] = curr;
    prev[key] = copyChildren(input)(key);
    return prev;
  }, {});

  const children = new Set();
  Object.entries(input).forEach(([key, data]) => {
    if (data.children.length > 0) {
      data.children.forEach(child => children.add(child));
    } else {
      children.add(key);
    }
  });
  children.forEach(key => delete populatedMap[key]);

  return Object.entries(populatedMap).reduce((prev, curr) => {
    const [key, data] = curr;
    return children.has(key) ? prev : Object.assign(prev, { [key]: data });
  }, {});
};

console.log('output', createTreeMap(sanitizedInput));
