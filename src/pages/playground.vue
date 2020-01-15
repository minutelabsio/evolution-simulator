<template lang="pug">
.playground
  .upper
    .columns.is-gapless
      .column(ref="resultsContainer")
        .viewer
          .screen
            //- canvas.stage(ref="canvas", v-bind="simulationProps")
            GenerationViewer.viewer(:generation-index="genIndex", :step-time="stepTime")

          .controls
            AudioScrubber(:progress="progress", @scrub="onScrub")
            br/
            b-field(grouped, position="is-centered")
              b-field
                b-button.btn-dark(@click="prevGeneration()")
                  b-icon(icon="skip-previous")
              b-field
                b-button.btn-dark(@click="togglePlay()")
                  b-icon(:icon="paused ? 'play' : 'pause'")
              b-field
                b-button.btn-dark(@click="nextGeneration()")
                  b-icon(icon="skip-next")

              //- .food(v-for="(food, index) in foodElements", :style="food.style", :class="{'is-eaten': food.isEaten}", :key="'food'+index")
              //- .creature(v-for="(creature, index) in creatures", :style="creature.style", :key="index")

          //- .columns.is-centered
          //-   .column
          //-     h2.is-size-2.has-text-centered Population
          //-     TraitChart(v-if="simulation", :data="populationData", label="Population")
          //-
          //- .columns.is-multiline
          //-   .column.is-half(v-for="trait in traitData")
          //-     h2.is-size-2.has-text-centered {{ trait.label }}
          //-     TraitChart(v-if="simulation", :data="trait.data", :label="trait.label", :key="trait.label")

      .column.is-one-third
        .section
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
                b-button.button.is-primary.is-large(@click="run", :loading="isLoading") Run!


  .bottom-drawer
    Legend.legend(:data="flowerLegend", @select="propertySelect($event.index)")
    .generation-selector(:class="{ 'is-finished': !canContinue }")
      FlowerTimeline(
        v-model="genIndex"
        , :data="flowerTimelineData"
        , :data-ranges="flowerRanges"
        , :colors="flowerColors"
        , :topPetal="topPetal"
        , @dataSelect="genIndex === $event.generation && propertySelect($event.selected.index)"
      )
        template(v-if="canContinue", #after)
          b-button.btn-dark(@click="continueSimulation") Load More
</template>

<script>
// import _throttle from 'lodash/throttle'
import { mapGetters } from 'vuex'
import chroma from 'chroma-js'
import Copilot from 'copilot'
import AudioScrubber from '@/components/audio-scrubber'
import TraitChart from '@/components/trait-plot'
import FlowerChart from '@/components/flower-chart'
import FlowerTimeline from '@/components/flower-timeline'
import GenerationViewer from '@/components/generation-viewer'
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
    drawCircle(ctx, {x, y}, 2, isEaten ? '#111' : '#666')
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
  return value[0] !== undefined || value
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
    , AudioScrubber
    , GenerationViewer
  }
  , provide(){
    const self = this
    return {
      getTime(){ return self.time }
      , getStep(){ return self.time / self.stepTime }
    }
  }
  , data: () => ({
    paused: true
    , canvasScale: 1

    , flowerColors: {
      center: '#e6e6e6'
      , petals: chroma.scale('Set1').colors(8)
    }
    , topPetal: 0

    , cfg: {
      seed: 118
      , food_per_generation: 50
      , max_generations: 50
      , size: 500
    }

    , creatureProps: {
      speed: [6, 1]
      , sense_range: [50, 10]
      , reach: [5, 1]
      , life_span: [4, 1]
      , energy: 500
    }

    , creatureCount: 50

    , stepTime: 100 // ms
    , genIndex: 0

    , time: 0
    , progress: 0
    // , traitToColor: 'speed'
  })
  , created(){
    this.player = Copilot.Player({ totalTime: 1 })
    this.cfg = this.config
    this.creatureProps = this.creatureConfig.template
  }
  , mounted(){
    this.canvasScale = Math.min(this.$refs.resultsContainer.offsetWidth, this.$refs.resultsContainer.offsetHeight) / this.size
    // this.canvas = this.$refs.canvas
    // this.ctx = this.canvas.getContext('2d')

    this.player.on('animate', () => {
      this.time = this.player.time
      this.progress = this.time/this.totalTime * 100
      // this.draw()
    })
    this.player.on('togglePause', () => {
      if ( this.player.paused !== this.paused ){
        this.paused = this.player.paused
      }
    })

    this.$nextTick(() => {
      this.run()
    })
  }
  , beforeDestroy(){
    this.player.togglePause(true)
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
      }
      , deep: true
    }
    , creatureCount(){
      this.$store.dispatch('simulation/setCreatureConfig', {
        count: this.creatureCount
        , template: this.creatureProps
      })
    }
    , genIndex(){
      this.player.seek(0)
      this.$nextTick(() => {
        this.paused = false
      })
    }
    , simulation(){
      this.genIndex = 0
      this.player.seek(0)
      this.$nextTick(() => {
        this.paused = false
      })
    }
  }
  , computed: {
    simulation(){
      return this.getResults()
    }
    , size(){ return this.cfg.size }
    , simulationCfg(){
      return {
        size: this.cfg.size
        , seed: this.cfg.seed | 0
        , food_per_generation: this.cfg.food_per_generation | 0
        , max_generations: this.cfg.max_generations | 0
      }
    }
    , simulationProps(){
      const ratio = window.devicePixelRatio * this.canvasScale
      return {
        width: this.size * ratio
        , height: this.size * ratio
        , style: {
          width: this.size * this.canvasScale + 'px'
          , height: this.size * this.canvasScale + 'px'
        }
      }
    }
    , traitToColor(){
      return creatureTraits[this.topPetal]
    }
    , traitColor(){
      return this.flowerColors.petals[this.topPetal]
    }
    , traitScale(){
      let scale = chroma.scale(['white', this.traitColor])
      if ( this.simulation ){
        let stat = this.stats[this.traitToColor]
        let min = stat.min()
        let max = stat.max()
        return scale.domain([min, max])
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
    , ...mapGetters('simulation', {
      getResults: 'getResults'
      , canContinue: 'canContinue'
      , isLoading: 'isLoading'
      , config: 'config'
      , creatureConfig: 'creatureConfig'
    })
  }
  , methods: {
    run(){
      this.$store.dispatch('simulation/run')
    }
    , continueSimulation(){
      this.$store.dispatch('simulation/continue')
    }
    , togglePlay(){
      this.paused = !this.paused
    }
    , prevGeneration(){
      if ( this.genIndex <= 0 ){ return }
      this.player.togglePause(true)
      this.player.seek(0)
      this.genIndex -= 1
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
    , onScrub(progress){
      this.updateTime(progress * this.totalTime / 100)
    }
    , draw(){
      if (!this.generation){ return }
      const ctx = this.ctx
      const step = this.currentStep
      const gen = this.generation
      const ratio = window.devicePixelRatio
      const scale = ratio * this.canvasScale
      ctx.clearRect(0, 0, this.size, this.size)
      ctx.setTransform(scale, 0, 0, scale, 0, 0);
      drawFood(ctx, step, gen.food)
      drawCreatures(ctx, this.stepTime, this.time, gen.creatures, this.traitToColor, this.traitScale)
    }
    , propertySelect(index){
      if ( index !== undefined && index < creatureTraits.length ){
        this.topPetal = index
      }
    }
  }
}
</script>

<style lang="sass" scoped>
.playground
  // overflow: auto
  height: 100%
  display: flex
  flex-direction: column
.upper
  flex: 1
  overflow: hidden

  .columns
    height: 100%
.bottom-drawer
  background: $black-ter
  border-top: 1px solid $black
  min-height: 261px
.viewer
  display: flex
  height: 100%
  flex-direction: column
  .screen
    flex-grow: 1
    overflow: hidden
.stage
  display: none
  position: relative
  // border: 1px solid rgba(255, 255, 255, 0.4)
  overflow: hidden
  background: $black-bis
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
.generation-selector.is-finished
  >>> .flower-timeline .inner:after
    content: ''
    background: center center url('https://cdn0.iconfinder.com/data/icons/nature-and-environment-1/64/skeleton-pirate-crossbones-danger-deadly-skull-512.png') no-repeat
    background-size: contain
    width: 120px
    flex: 0 0 auto
</style>
