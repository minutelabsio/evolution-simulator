<template lang="pug">
.playground
  .simulation-controls(:class="{ collapsed: !toolbar }")
    button.collapse-bar(@click="toolbar = !toolbar")
      b-icon(:icon="toolbar ? 'menu-right' : 'menu-left'")

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

  .upper
    router-view

  .bottom-drawer
    .generation-selector(:class="{ 'is-finished': !canContinue }")
      FlowerTimeline(
        v-model="genIndex"
        , :data="flowerTimelineData"
        , :data-ranges="flowerRanges"
        , :colors="flowerColors"
        , :topPetal="topPetal"
        , @dataSelect="genIndex === $event.generation && propertySelect($event.selected.index)"
      )
        template(#before)
          .flower-timeline-spacer
        template(v-if="canContinue", #after)
          b-button.btn-dark(@click="continueSimulation") Load More
          .flower-timeline-spacer
    Legend.legend(:data="flowerLegend", @select="propertySelect($event.index)")
</template>

<script>
// import _throttle from 'lodash/throttle'
import { mapGetters } from 'vuex'
import chroma from 'chroma-js'
import TraitChart from '@/components/trait-plot'
import FlowerChart from '@/components/flower-chart'
import FlowerTimeline from '@/components/flower-timeline'
import Legend from '@/components/legend'

const creatureTraits = ['speed', 'sense_range', 'reach', 'life_span', 'age']

export default {
  name: 'Simulation'
  , props: {
  }
  , components: {
    TraitChart
    , FlowerChart
    , FlowerTimeline
    , Legend
  }
  , data: () => ({
    toolbar: true

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
    // , traitToColor: 'speed'
  })
  , created(){
    this.cfg = this.config
    this.creatureProps = this.creatureConfig.template
  }
  , mounted(){
    this.$nextTick(() => {
      this.run()
    })
  }
  , watch: {
    simulationCfg: {
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

  }
  , computed: {
    size(){ return this.cfg.size }
    , simulationCfg(){
      return {
        size: this.cfg.size
        , seed: this.cfg.seed | 0
        , food_per_generation: this.cfg.food_per_generation | 0
        , max_generations: this.cfg.max_generations | 0
      }
    }
    , traitToColor(){
      return creatureTraits[this.topPetal]
    }
    , traitColor(){
      return this.flowerColors.petals[this.topPetal]
    }
    , generationStats(){
      if ( !this.stats ){ return [] }
      return this.stats.generation_statistics
    }
    , flowerData(){
      if (!this.stats){ return {} }
      let g = this.generationStats[this.genIndex]
      return {
        center: g.population
        , petals: creatureTraits.map(k => g[k].mean)
      }
    }
    , flowerTimelineData(){
      if (!this.stats){ return [] }
      let stats = this.generationStats
      return stats.map(v => ({
        center: v.population
        , petals: creatureTraits.map(k => v[k].mean)
      }))
    }
    , flowerRanges(){
      if ( !this.stats ){ return {} }
      let { population } = this.stats
      return {
        center: [population.min, population.max]
        , petals: creatureTraits.map(k => this.stats[k]).map(t => [t.min, t.max])
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
    , genIndex: {
      get(){
        return this.generationIndex
      }
      , set(v){
        this.loadGeneration(v)
      }
    }
    , generation(){
      return this.getCurrentGeneration()
    }
    , ...mapGetters('simulation', {
      canContinue: 'canContinue'
      , isLoading: 'isLoading'
      , config: 'config'
      , creatureConfig: 'creatureConfig'
      , getCurrentGeneration: 'getCurrentGeneration'
      , generationIndex: 'currentGenerationIndex'
      , stats: 'statistics'
    })
  }
  , methods: {
    run(){
      this.$store.dispatch('simulation/run')
    }
    , continueSimulation(){
      this.$store.dispatch('simulation/continue')
    }
    , loadGeneration(v){
      this.$store.dispatch('simulation/loadGeneration', v)
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
  // overflow: hidden
  background: #333333
.bottom-drawer
  background: $black-ter
  border-top: 1px solid $black
  min-height: 261px

.simulation-controls
  position: absolute
  top: 0
  right: 0
  margin-top: 10vh
  z-index: 1
  width: 460px
  padding: 1.5em
  padding-left: 2em
  border-radius: 4px 0 0 4px
  border: 1px solid darken($grey, 10)
  border-right: none
  background: rgba(0, 0, 0, 0.3)
  backdrop-filter: blur(3px)
  box-shadow: 2px 2px 3px rgba(0, 0, 0, 0.3)

  transition: transform 0.4s ease

  &.collapsed
    transform: translate3d(440px, 0, 0)

  .collapse-bar
    position: absolute
    top: 0
    left: 0
    bottom: 0
    width: 20px
    border: none
    overflow: hidden
    background: transparentize($background, 0.1)
    padding: 0
    border-radius: 4px 0 0 4px
    color: $text
    cursor: pointer
    outline: none
    transition: background 0.15s ease
    &:hover
      background: $background
    &:active
      background: lighten($background, 2)
    .icon
      width: 100%

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

.generation-selector
  >>> .flower-timeline
    padding-top: 60px

    .generation.selected
      position: relative
      &:before
        content: ''
        position: absolute
        top: -100px
        left: 50%
        margin-left: -50px
        border: 50px solid transparent
        border-color: transparent transparent transparentize($blue, 0.8) transparent
  .flower-timeline-spacer
    flex: 0 0 auto
    width: calc(50vw - 50px)
.generation-selector.is-finished
  >>> .flower-timeline .inner:after
    content: ''
    background: center center url('https://cdn0.iconfinder.com/data/icons/nature-and-environment-1/64/skeleton-pirate-crossbones-danger-deadly-skull-512.png') no-repeat
    background-size: contain
    width: 120px
    flex: 0 0 auto
</style>
