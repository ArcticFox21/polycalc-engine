const { optim, calc } = require('./fightEngine')
const mock = require('./mockData')

for (data of mock) {
  const now = Date.now()
  const res = optim(data.attackers, data.defender)
  console.log(data.type, (Date.now() - now), 'ms')
}

// const now = Date.now()
// const res = optim(mock[9].attackers, mock[9].defender)
// console.log(mock[9].type, Date.now() - now, 'ms')

