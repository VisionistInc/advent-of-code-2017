const input = 361527;

var calcDistance = input => {
  const nearestPerfectSquareRoot = Math.round(Math.sqrt(input));

  const nearestPerfectSquare =
    nearestPerfectSquareRoot * nearestPerfectSquareRoot;

  const isEven = nearestPerfectSquareRoot % 2 === 0;

  const roundedUp = Math.sqrt(input) % nearestPerfectSquareRoot > 0.5;

  const nearestPerfectSquareRootCoordinates = isEven
    ? [-(nearestPerfectSquareRoot / 2 - 1), nearestPerfectSquareRoot / 2]
    : [(nearestPerfectSquareRoot - 1) / 2, -(nearestPerfectSquareRoot - 1) / 2];

  let [x, y] = nearestPerfectSquareRootCoordinates;

  if (!isEven && !roundedUp) {
    x++;
    for (let i = nearestPerfectSquare + 1; i < input; i++) {
      if (i === input) break;
      y++;
    }
  } else if (!isEven && roundedUp) {
    for (let i = nearestPerfectSquare; i > input; i--) {
      if (i === input) break;
      x--;
    }
  } else if (isEven && !roundedUp) {
    x--;
    for (let i = nearestPerfectSquare + 1; i < input; i++) {
      if (i === input) break;
      y--;
    }
  } else {
    for (let i = nearestPerfectSquare; i > input; i--) {
      if (i === input) break;
      x++;
    }
  }

  return Math.abs(x) + Math.abs(y);
};

console.log('output:', calcDistance(input));
