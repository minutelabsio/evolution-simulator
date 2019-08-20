<template lang="pug">
.playground
  .container
    .section
      .content(v-if="simulation")
        b-field
          b-select(v-model="genIndex")
            option(v-for="index in simulation.generations.length", :value="index - 1") {{ index - 1 }}
        b-slider(v-model="time", :max="totalTime")
        .creatures(:style="simulationStyle")
          .creature(v-for="(creature, index) in creatures", :style="creature.style", :key="index")
</template>

<script>
import Copilot from 'copilot'
import createWorker from '@/workers/simulation'
const worker = createWorker()

function lerpArray(from, to, t){
  return Copilot.Interpolators.Array(from, to, t)
}

const creaturePositionAt = stepFrac => creature => {
  let step = Math.floor(stepFrac)
  let frac = stepFrac % step
  let from = creature.movement_history[step]
  if ( !frac ){ return from }

  let to = creature.movement_history[step + 1]
  return lerpArray(from, to, frac)
}

function draw(generation, stepTime, now){
  let stepFrac = now / stepTime
  let creaturePositions = generation.creatures.map(creaturePositionAt(stepFrac))

  return creaturePositions.map(p => {
    let [x,y] = p
    let transform = `translate3d(${x}px, ${y}px, 0)`
    return { style: { transform } }
  })
}

export default {
  name: 'Playground'
  , props: {
  }
  , components: {
  }
  , data: () => ({
    size: 500
    , stepTime: 500 //
    , genIndex: 0

    , time: 0
    , simulation: null
  })
  , mounted(){
    worker.runSimulation({
      size: this.size
      , seed: 123
      , creature_count: 10
      , food_per_generation: 10
      , max_generations: 10
      , behaviours: [{ name: 'BasicMoveBehaviour' }]
    }).then( simulation => {
      this.simulation = simulation
    })
  }
  , computed: {
    simulationStyle(){
      return {
        width: this.size + 'px'
        , height: this.size + 'px'
      }
    }
    , creatures(){
      return draw(this.simulation.generations[this.genIndex], this.stepTime, this.time)
    }
    , generation(){
      return this.simulation.generations[this.genIndex]
    }
    , totalTime(){
      return this.stepTime * this.generation.steps
    }
  }
}
</script>

<style lang="sass" scoped>
.creatures
  position: relative
  border: 1px solid rgba(255, 255, 255, 0.4)
  overflow: hidden
  .creature
    position: absolute
    top: 0
    left: 0
    width: 10px
    height: 10px
    background: $red
</style>
