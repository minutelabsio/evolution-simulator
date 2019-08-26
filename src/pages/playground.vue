<template lang="pug">
.playground
  .container
    .section
      .content(v-if="simulation")
        b-field
          b-select(v-model="genIndex")
            option(v-for="index in simulation.generations.length", :value="index - 1") Generation {{ index - 1 }}
        b-field(grouped)
          b-field
            b-button.btn-dark(@click="togglePlay()")
              b-icon(:icon="paused ? 'play' : 'pause'")
          b-field
            b-button.btn-dark(@click="nextGeneration()")
              b-icon(icon="skip-next")
          b-slider(@input="updateTime", :value="time", :max="totalTime", rounded)
        b-field
          b-select(v-model="traitToColor")
            option(value="speed") Monitor Speed
            option(value="sense_range") Monitor Sense Range
        .stage(:style="simulationStyle")
          .food(v-for="(food, index) in foodElements", :style="food.style", :class="{'is-eaten': food.isEaten}", :key="'food'+index")
          .creature(v-for="(creature, index) in creatures", :style="creature.style", :key="index")
</template>

<script>
import chroma from 'chroma-js'
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

export default {
  name: 'Playground'
  , props: {
  }
  , components: {
  }
  , data: () => ({
    paused: true

    , size: 500
    , stepTime: 20 // ms
    , genIndex: 0

    , time: 0
    , simulation: null
    , traitToColor: 'speed'
  })
  , created(){
    this.player = Copilot.Player({ totalTime: 1 })
  }
  , mounted(){
    let start = performance.now()
    worker.runSimulation({
      size: this.size
      , seed: 124
      , creature_count: 20
      , food_per_generation: 25
      , max_generations: 50
      , behaviours: [
        { name: 'WanderBehaviour' }
        , { name: 'ScavengeBehaviour' }
        , { name: 'BasicMoveBehaviour' }
        , { name: 'HomesickBehaviour' }
        , { name: 'StarveBehaviour' }
      ]
    }).then( simulation => {
      let time = performance.now() - start;
      console.log(`Computed in ${time}ms`)
      this.simulation = simulation
      console.log(simulation)
    })

    this.player.on('update', () => {
      this.time = this.player.time
    })
    this.player.on('togglePause', () => {
      if ( this.player.paused !== this.paused ){
        this.paused = this.player.paused
      }
    })
  }
  , beforeDestroy(){
    this.player.off(true)
  }
  , watch: {
    totalTime(){
      this.player.totalTime = this.totalTime
    }
    , paused(){
      this.player.togglePause(this.paused)
    }
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
        let [x,y] = f.position
        let transform = `translate3d(${x}px, ${y}px, 0)`
        let isEaten = f.status.Eaten < step
        return { style: { transform }, isEaten }
      })
    }
    , traitScale(){
      let scale = chroma.scale('OrRd')
      if ( this.simulation ){
        let max = this.simulation.generations.reduce((max, g) => {
          let newmax = g.creatures.reduce((max, c) => Math.max(max, c[this.traitToColor][0]), max)
          return Math.max(max, newmax)
        }, 0)
        return scale.domain([0, max])
      }
      return scale
    }
    , creatures(){
      return this.getCreatureProps(this.simulation.generations[this.genIndex], this.stepTime, this.time)
    }
    , generation(){
      if ( !this.simulation ){ return null }
      return this.simulation.generations[this.genIndex]
    }
    , totalTime(){
      if ( !this.generation ){ return 1 }
      return this.stepTime * this.generation.steps
    }
  }
  , methods: {
    togglePlay(){
      this.paused = !this.paused
    }
    , nextGeneration(){
      if ( this.genIndex >= (this.simulation.generations.length - 1) ){ return }
      this.player.togglePause(true)
      this.player.seek(0)
      this.genIndex += 1
    }
    , updateTime(time){
      this.player.seek(time)
    }
    , getCreatureProps(generation, stepTime, now){
      let stepFrac = now / stepTime
      let creaturePositions = generation.creatures.map(creaturePositionAt(stepFrac))

      return creaturePositions.map((p, i) => {
        let creature = generation.creatures[i]
        let [x,y] = p
        let transform = `translate3d(${x}px, ${y}px, 0)`
        let color = this.traitScale(creature[this.traitToColor][0]).css()

        return { style: { transform, backgroundColor: color } }
      })
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
