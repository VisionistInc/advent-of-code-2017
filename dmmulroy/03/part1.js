const input = 361527;

const nearestPerfectSquare = Math.round(Math.sqrt(input));

const nearestPerfectSquareCoordinates =
  nearestPerfectSquare % 2 === 0
    ? [-(nearestPerfectSquare / 2 - 1), nearestPerfectSquare / 2]
    : [(nearestPerfectSquare - 1) / 2, -(nearestPerfectSquare - 1) / 2];
