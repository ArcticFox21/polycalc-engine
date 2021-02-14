const { generateArraySequences, generateSequences, multicombat, evaluate } = require('./sequencer')

module.exports.optim = function (attackers, defender) {
  const arrayNbAttackers = generateArraySequences(attackers.length)
  const sequences = generateSequences(arrayNbAttackers)
  let solutions = []

  sequences.forEach(function (sequence) {
    const attackersSorted = []

    for (let j = 0; j < sequence.length; j++) {
      attackersSorted.push(attackers[sequence[j] - 1]);
    }

    const solution = multicombat(attackersSorted, defender, sequence)

    solutions.push(solution)
  })

  let bestSolution = solutions[0]

  solutions.forEach((solution) => {
    if (evaluate(bestSolution, solution))
      bestSolution = solution
  })

  if (bestSolution.defenderHP === defender.currenthp)
    throw `No unit can make a dent in this ${defender.name}${defender.description}...`

  let defHP = defender.currenthp
  bestSolution.finalSequence.forEach((seqIndex, order) => {
    seqIndex--
    defHP = defHP - bestSolution.hpDealt[order]
    attackers[seqIndex].defHP = defHP
  })

  return 'Done'
}

module.exports.calc = function (attackers, defender) {
  const sequence = []
  for (let i = 1; i <= attackers.length; i++) {
    sequence.push(i)
  }

  const attackersSorted = []

  for (let j = 0; j < sequence.length; j++) {
    attackersSorted.push(attackers[sequence[j] - 1]);
  }

  const solution = multicombat(attackersSorted, defender, sequence)

  let defHP = defender.currenthp
  solution.finalSequence.forEach((seqIndex, order) => {
    seqIndex--
    defHP = defHP - solution.hpDealt[order]
  })

  return 'Done'
}