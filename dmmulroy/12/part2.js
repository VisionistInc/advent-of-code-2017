const fs = require('fs');
const path = require('path');

const input = fs.readFileSync(path.join(__dirname, 'input.txt'), 'utf8');

const sanitizedInput = input
  .trim()
  .split('\n')
  .map(row =>
    row
      .split(' <-> ')[1]
      .split(', ')
      .map(Number)
  );

const dfs = (adjList, vertex, marked) => {
  marked[vertex] = true;
  adjList[vertex].forEach(connectedVertex => {
    if (!marked[connectedVertex]) {
      dfs(adjList, connectedVertex, marked);
    }
  });
};

const countVertices = (adjList, sourceVertex) => {
  let marked = [];
  dfs(adjList, sourceVertex, marked);

  return marked;
};

const countGroups = adjList => {
  const groupsByVertex = adjList
    .map((_, idx) => countVertices(adjList, idx))
    .map(JSON.stringify);

  return groupsByVertex.reduce((prev, curr) => {
    prev.add(curr);
    return prev;
  }, new Set()).size;
};

console.log('output:', countGroups(sanitizedInput));
