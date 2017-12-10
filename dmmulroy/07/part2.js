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

  return Object.entries(populatedMap).reduce((prev, curr) => {
    const [key, data] = curr;
    return children.has(key) ? prev : Object.assign(prev, { [key]: data });
  }, {});
};

const treeMap = createTreeMap(sanitizedInput);

const root = Object.keys(treeMap)[0];

const sumChildrenWeights = ({ weight, children }) => {
  if (children.length === 0) {
    return { weight, totalWeight: Number(weight) };
  } else {
    return {
      weight,
      totalWeight: children.reduce(
        (prev, curr) => prev + sumChildrenWeights(curr).totalWeight,
        Number(weight)
      )
    };
  }
};

const calcAdjustment = summedWeights =>
  summedWeights.reduce((prev, curr, idx) => {
    const { weight, totalWeight } = curr;

    if (prev[totalWeight]) {
      prev[totalWeight]++;
    } else {
      prev[totalWeight] = 1;
    }

    if (idx === summedWeights.length - 1) {
      const adjustmentKey = Number(
        Object.keys(prev).filter(key => prev[key] === 1)[0]
      );

      delete prev[adjustmentKey];
      const otherKey = Number(Object.keys(prev)[0]);
      return adjustmentKey > otherKey
        ? [adjustmentKey - otherKey, adjustmentKey]
        : [otherKey - adjustmentKey, adjustmentKey];
    }

    return prev;
  }, {});

const findUnbalancedDisc = treeMap => {
  const summedWeights = treeMap.children.map(sumChildrenWeights);
  const [adjustment, keyToAdjust] = calcAdjustment(summedWeights);
  return summedWeights.reduce((prev, curr) => {
    if (curr.totalWeight == keyToAdjust) {
      return curr.weight - adjustment;
    }
    return prev;
  }, 0);
};

console.log('output', findUnbalancedDisc(createTreeMap(sanitizedInput)[root]));
