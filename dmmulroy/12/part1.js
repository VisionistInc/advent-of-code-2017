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

  return marked.filter(Boolean).length;
};

console.log('output:', countVertices(sanitizedInput, 0));
