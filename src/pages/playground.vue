<template lang="pug">
.playground
  .container
    .section
      .content(v-if="simulation")
        b-field
          b-select(v-model="genIndex")
            option(v-for="index in simulation.generations.length", :value="index - 1") {{ index - 1 }}
        b-slider(v-model="time", :max="totalTime")
        .stage(:style="simulationStyle")
          .food(v-for="(food, index) in foodElements", :style="food.style", :class="{'is-eaten': food.isEaten}", :key="'food'+index")
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
  let frac = stepFrac % Math.max(1, step)
  let from = creature.movement_history[step] || creature.movement_history[creature.movement_history.length - 1]
  let to = creature.movement_history[step + 1]

  if ( !to ){ return from }
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
      , behaviours: [
        { name: 'WanderBehaviour' }
        , { name: 'ScavengeBehaviour' }
        , { name: 'BasicMoveBehaviour' }
      ]
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
    , currentStep(){
      return Math.floor(this.time / this.stepTime)
    }
    , foodElements(){
      let step = this.currentStep
      return this.generation.food.map(f => {
        let [x,y] = f[0]
        let transform = `translate3d(${x}px, ${y}px, 0)`
        let isEaten = f[1] !== 'Available'
        return { style: { transform }, isEaten }
      })
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
.stage
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
  .food
    position: absolute
    top: 0
    left: 0
    width: 4px
    height: 4px
    background: $green
    &.is-eaten
      background: $grey
  .creature,
  .food
    border-radius: 50%
</style>
