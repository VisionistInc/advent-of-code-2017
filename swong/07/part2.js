const input = document.body.textContent.trim().split('\n');
const parse = s => s.match(/(\w+) \((\d+)\)(?: -> (.+))?/);
const towerMap = input.map(parse).reduce((o, [, key, w, nodes]) =>
  Object.assign(o, { [key]: { key, weight: +w, towers: nodes && nodes.split(', ') } }), {});

calcWeight = value => {
  const node = towerMap[value];
  // base case, return weight
  if (!node.towers) return node.weight;
  else {
    // but I want to check these returned tower weights
    const towerWeights = node.towers.map(tower => {
      return calcWeight(tower);
    });

    // check if array values equal
    const result = !!towerWeights.reduce((a, b) => {
      return (a === b) ? a : NaN
    });

    // if result === true, the imbalance does not lie here
    if (result) {
      return towerWeights[0] * towerWeights.length + node.weight;
    } else {
      // the imbalance lies around here
      const sum = towerWeights.reduce((prev, curr) => curr + prev);
      console.log('Unbalanced Node: ', node)
      for(let i=0; i<node.towers.length; i++) {
        console.log(node.towers[i] + ': ' + towerWeights[i])
      }
      return sum + node.weight;
    }
  }
}

// look in the console to determine the node that is unbalanced and adjust it's weight accordingly
calcWeight('ahnofa')
