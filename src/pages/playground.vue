<template lang="pug">
.playground
  .container
    .section
      h3.title.is-size-3 Setup
      .columns
        .column
          h4.title.is-size-5 Initial Creature Properties
          b-field(grouped)
            b-field(label="Energy")
              b-input(v-model="creatureProps.energy", type="number", min="0", step="any")
          b-field(grouped)
            b-field(label="Speed")
              b-input(v-model="creatureProps.speed[0]", type="number", min="0", step="any")
            b-field(label="Speed mutation variance")
              b-input(v-model="creatureProps.speed[1]", type="number", min="0", step="any")
          b-field(grouped)
            b-field(label="Sense")
              b-input(v-model="creatureProps.sense_range[0]", type="number", min="0", step="any")
            b-field(label="Sense mutation variance")
              b-input(v-model="creatureProps.sense_range[1]", type="number", min="0", step="any")
          b-field(grouped)
            b-field(label="Reach")
              b-input(v-model="creatureProps.reach[0]", type="number", min="0", step="any")
            b-field(label="Reach mutation variance")
              b-input(v-model="creatureProps.reach[1]", type="number", min="0", step="any")
          b-field(grouped)
            b-field(label="Avg Lifespan")
              b-input(v-model="creatureProps.life_span[0]", type="number", min="0", step="any")
            b-field(label="Avg Lifespan mutation variance")
              b-input(v-model="creatureProps.life_span[1]", type="number", min="0", step="any")
        .column
          b-field(grouped)
            b-field(label="Number of Creatures")
              b-input(v-model="creatureCount", type="number", min="1", step="1")
            b-field(label="Food Per Generation")
              b-input(v-model="cfg.food_per_generation", type="number", min="0", step="1")
          b-field(grouped)
            b-field(label="Random Seed")
              b-input(v-model="cfg.seed", type="number", min="0", step="1")
            b-field(label="Max Generations")
              b-input(v-model="cfg.max_generations", type="number", min="0", step="1")
          b-field
            b-button.button.is-primary.is-large(@click="run", :loading="calculating") Run!

    .section
      h4.title.is-size-4 Simulation Results
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
            option(value="reach") Monitor Reach
            option(value="life_span") Lifespan
            option(value="age") Age
          .scale
            .min(:style="{ backgroundColor: traitScale(0) }")
              span 0
            .max(:style="{ backgroundColor: traitScale(traitScale.domain()[1]) }")
              span {{ traitScale.domain()[1].toFixed(2) }}
          //- .food(v-for="(food, index) in foodElements", :style="food.style", :class="{'is-eaten': food.isEaten}", :key="'food'+index")
          //- .creature(v-for="(creature, index) in creatures", :style="creature.style", :key="index")

      .columns
        .column.is-full
          Legend.legend(:data="flowerLegend", @select="topPetal = $event.index")
          FlowerTimeline(
            v-model="genIndex"
            , :data="flowerTimelineData"
            , :data-ranges="flowerRanges"
            , :colors="flowerColors"
            , :topPetal="topPetal"
          )
          //- FlowerChart(:width="100", :height="100", :data="flowerData", :data-ranges="flowerRanges")
      .columns.is-centered
        .column
          canvas.stage(ref="canvas", v-bind="simulationProps")
        .column
          h2.is-size-2.has-text-centered Population
          TraitChart(v-if="simulation", :data="populationData", label="Population")

      .columns.is-multiline
        .column.is-half(v-for="trait in traitData")
          h2.is-size-2.has-text-centered {{ trait.label }}
          TraitChart(v-if="simulation", :data="trait.data", :label="trait.label", :key="trait.label")
</template>

<script>
// import _times from 'lodash/times'
import chroma from 'chroma-js'
import Copilot from 'copilot'
import TraitChart from '@/components/trait-plot'
import FlowerChart from '@/components/flower-chart'
import FlowerTimeline from '@/components/flower-timeline'
import Legend from '@/components/legend'
import { RunningStatistics } from '@/lib/math'

function lerpArray(from, to, t){
  return Copilot.Interpolators.Array(from, to, t)
}

const creaturePositionAt = stepFrac => creature => {
  let step = Math.floor(stepFrac)
  let frac = stepFrac % Math.max(1, step)
  let from = creature.movement_history[step] || creature.movement_history[creature.movement_history.length - 1]
  let to = creature.movement_history[step + 1]

  if ( !to ){ return from }
  // frac = Copilot.Easing.Quadratic.InOut(frac)
  return lerpArray(from, to, frac)
}

function drawCircle(ctx, {x, y}, radius, color = 'white'){
  ctx.beginPath()
  ctx.arc(x, y, radius, 0, 2 * Math.PI, false)
  ctx.fillStyle = color
  ctx.fill()
}

function drawFood(ctx, step, food){
  food.forEach(f => {
    let [x,y] = f.position
    let isEaten = f.status.Eaten < step
    drawCircle(ctx, {x, y}, 2, isEaten ? '#444' : 'white')
  })
}

function drawCreatures(ctx, stepTime, now, creatures, traitName, scale){
  let stepFrac = now / stepTime
  const getCreaturePosition = creaturePositionAt(stepFrac)

  creatures.forEach(c => {
    let [x,y] = getCreaturePosition(c)
    let trait = getTrait(c, traitName)
    let color = scale(trait).css()

    drawCircle(ctx, {x, y}, 5, color)
  })
}

// function getAverageTraitValue(generation, trait){
//   let len = generation.creatures.length
//   return generation.creatures.reduce((avg, c) => {
//     return avg + getTrait(c, trait)/len
//   }, 0)
// }

function getTrait(creature, trait){
  let value = creature[trait]
  return value[0] || value
}

const creatureTraits = ['speed', 'sense_range', 'reach', 'life_span', 'age']

export default {
  name: 'Playground'
  , props: {
  }
  , components: {
    TraitChart
    , FlowerChart
    , FlowerTimeline
    , Legend
  }
  , data: () => ({
    paused: true
    , calculating: false

    , flowerColors: {
      center: '#e6e6e6'
      , petals: chroma.scale('Set1').colors(8)
    }
    , topPetal: 0

    , cfg: {
      seed: 124
      , food_per_generation: 50
      , max_generations: 50
      , behaviours: [
        { name: 'WanderBehaviour' }
        , { name: 'ScavengeBehaviour' }
        , { name: 'BasicMoveBehaviour' }
        , { name: 'HomesickBehaviour' }
        , { name: 'StarveBehaviour' }
      ]
    }

    , creatureProps: {
      speed: [6, 1]
      , sense_range: [50, 10]
      , reach: [5, 1]
      , life_span: [4, 1]
      , energy: 500
    }

    , creatureCount: 50
    , size: 500

    , stepTime: 100 // ms
    , genIndex: 0

    , time: 0
    , traitToColor: 'speed'
  })
  , created(){
    this.player = Copilot.Player({ totalTime: 1 })
  }
  , mounted(){
    this.canvas = this.$refs.canvas
    this.ctx = this.canvas.getContext('2d')

    this.player.on('animate', () => {
      this.time = this.player.time
      this.draw()
    })
    this.player.on('togglePause', () => {
      if ( this.player.paused !== this.paused ){
        this.paused = this.player.paused
      }
    })

    this.$store.dispatch('simulation/randomizeCreatures').then(() => {
      this.run()
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
    , simulationCfg: {
      handler(cfg){
        this.$store.dispatch('simulation/setConfig', cfg)
      }
    }
    , creatureProps: {
      handler(cfg){
        this.$store.dispatch('simulation/setCreatureConfig', {
          count: this.creatureCount
          , template: cfg
        })
        this.$store.dispatch('simulation/randomizeCreatures')
      }
      , deep: true
    }
    , creatureCount(){
      this.$store.dispatch('simulation/setCreatureConfig', {
        count: this.creatureCount
        , template: this.creatureProps
      })
      this.$store.dispatch('simulation/randomizeCreatures')
    }
  }
  , computed: {
    simulationCfg(){
      return {
        size: this.size
        , seed: this.cfg.seed | 0
        , food_per_generation: this.cfg.food_per_generation | 0
        , max_generations: this.cfg.max_generations | 0
        , behaviours: this.cfg.behaviours
      }
    }
    , simulationProps(){
      return {
        width: this.size + 'px'
        , height: this.size + 'px'
      }
    }
    , currentStep(){
      return Math.floor(this.time / this.stepTime)
    }
    , traitScale(){
      let scale = chroma.scale('OrRd')
      if ( this.simulation ){
        let max = this.simulation.generations.reduce((max, g) => {
          let newmax = g.creatures.reduce((max, c) => Math.max(max, getTrait(c, this.traitToColor)), max)
          return Math.max(max, newmax)
        }, 0)
        return scale.domain([0, max])
      }
      return scale
    }
    , traitData(){
      if (!this.simulation){ return [] }
      return creatureTraits.map(t => {
        return {
          label: t
          , data: this.generationStats.map(gs => gs[t].mean())
        }
      })
    }
    , generationStats(){
      if (!this.simulation){ return [] }

      return this.simulation.generations.map(g => {
        let stats = {}
        creatureTraits.forEach(t => {
          stats[t] = RunningStatistics()
        })

        g.creatures.forEach(c => {
          creatureTraits.forEach(t => stats[t].push(getTrait(c, t)))
        })

        return stats
      })
    }
    , stats(){

      const generationStats = this.generationStats
      let population = RunningStatistics()
      let stats = { population }
      creatureTraits.forEach(t => {
        stats[t] = RunningStatistics()
      })

      if (!this.simulation){ return stats }

      this.simulation.generations.forEach((g, i) => {
        population.push(g.creatures.length)
        let s = generationStats[i]
        creatureTraits.forEach(t => {
          stats[t].push(s[t].mean())
        })
      })

      return stats
    }
    , populationData(){
      if (!this.simulation){ return [] }
      return this.simulation.generations.map(g => g.creatures.length)
    }
    , flowerData(){
      if (!this.simulation){ return {} }
      return {
        center: this.generation.creatures.length
        , petals: Object.values(this.generationStats[this.genIndex]).map(s => s.mean())
      }
    }
    , flowerTimelineData(){
      if (!this.simulation){ return [] }
      let stats = this.generationStats
      return this.simulation.generations.map((g, i) => ({
        center: g.creatures.length
        , petals: Object.values(stats[i]).map(s => s.mean())
      }))
    }
    , flowerRanges(){
      let { population, ...traits } = this.stats
      return {
        center: [population.min(), population.max()]
        , petals: Object.values(traits).map(t => [t.min(), t.max()])
      }
    }
    , flowerLegend(){
      return Object.values(creatureTraits)
        .map((name, i) => ({ name, color: this.flowerColors.petals[i] }))
        .concat([{
          name: 'population'
          , color: this.flowerColors.center
        }])
    }
    , generation(){
      if ( !this.simulation ){ return null }
      return this.simulation.generations[this.genIndex]
    }
    , totalTime(){
      if ( !this.generation ){ return 1 }
      return this.stepTime * this.generation.steps
    }
    , simulation(){
      return this.$store.getters['simulation/results']
    }
  }
  , methods: {
    run(){
      this.$store.dispatch('simulation/run')
    }
    , togglePlay(){
      this.paused = !this.paused
    }
    , nextGeneration(){
      if ( this.genIndex >= (this.simulation.generations.length - 1) ){ return }
      this.player.togglePause(true)
      this.player.seek(0)
      this.genIndex += 1
    }
    , updateTime(time){
      if ( time !== this.player.time ){
        this.player.seek(time)
      }
    }
    , draw(){
      if (!this.generation){ return }
      const ctx = this.ctx
      const step = this.currentStep
      const gen = this.generation
      ctx.clearRect(0, 0, this.size, this.size)
      drawFood(ctx, step, gen.food)
      drawCreatures(ctx, this.stepTime, this.time, gen.creatures, this.traitToColor, this.traitScale)
    }
  }
}
</script>

<style lang="sass" scoped>
.playground
  overflow: auto
  height: 100%
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

.scale
  display: flex
  width: 120px
  .min,
  .max
    flex: 1
    display: flex
    flex-direction: column
    justify-content: center
    text-align: center

  .min
    color: $grey-dark
.legend
  justify-content: center
  margin: 0.5em 0
</style>
