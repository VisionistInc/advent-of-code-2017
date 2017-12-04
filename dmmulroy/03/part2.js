const http = require('http');

const input = 361527;

const calcNextLargestValue = input => {
  http.get('http://oeis.org/A141481/b141481.txt', res => {
    let rawData = '';

    res.on('data', chunk => (rawData += chunk));
    res.on('end', () => {
      const splitData = rawData.trim().split('\n');
      const result = splitData
        .slice(2, splitData.length - 1)
        .map(val => Number(val.split(' ')[1]))
        .reduce((prev, curr) => {
          return curr > input && prev === 0 ? curr : prev;
        }, 0);
      console.log('result', result);
    });
  });
};

calcNextLargestValue(input);
