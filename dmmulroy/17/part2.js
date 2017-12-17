const steps = 394;

const getValueAfter0 = steps => {
  let currentPosition = 0;
  let value = 0;

  for (let iter = 1; iter < 50000000; iter++) {
    const nextPosition = (currentPosition + steps) % iter + 1;
    if (nextPosition === 1) value = iter;
    currentPosition = nextPosition;
  }

  return value;
};

console.log('output:', getValueAfter0(steps));
