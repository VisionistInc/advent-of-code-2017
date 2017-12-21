const steps = 394;

const getValueAfter2017 = steps => {
  let circularBuffer = [0];
  let currentPosition = 0;

  for (let iter = 1; iter < 2018; iter++) {
    const nextPosition = (currentPosition + steps) % circularBuffer.length + 1;
    circularBuffer.splice([nextPosition], 0, iter);
    currentPosition = nextPosition;
  }

  return circularBuffer[currentPosition + 1];
};

console.log('output:', getValueAfter2017(steps));
