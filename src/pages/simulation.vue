<template lang="pug">
.playground
  .logo
    img(src="@/assets/logo-dark.png")
  .upper
    router-view

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
        template(#before)
          .flower-timeline-spacer
        template(v-if="canContinue", #after)
          b-button.btn-dark(@click="continueSimulation") Load More
          .flower-timeline-spacer
</template>

<script>
// import _throttle from 'lodash/throttle'
import { mapGetters } from 'vuex'
import chroma from 'chroma-js'
import TraitChart from '@/components/trait-plot'
import FlowerChart from '@/components/flower-chart'
import FlowerTimeline from '@/components/flower-timeline'
import Drawer from '@/components/drawer'
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
    , Drawer
  }
  , data: () => ({
    toolbar: true

    , flowerColors: {
      center: '#e6e6e6'
      , petals: chroma.scale('Set1').colors(8)
    }
    , topPetal: 0
  })
  , created(){
  }
  , mounted(){
    this.$nextTick(() => {
      this.run()
    })
  }
  , watch: {

  }
  , computed: {
    traitToColor(){
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
.logo
  position: absolute
  top: 1em
  left: 1em
  pointer-events: none
  z-index: 10
  opacity: 0.7
  height: 40px
  overflow: hidden
  img
    width: 48px
.upper
  position: relative
  z-index: 2
  flex: 1
  // overflow: hidden
  background: #333333
.bottom-drawer
  position: relative
  background: $black-ter
  border-top: 1px solid $black
  // min-height: 213px
  .legend
    position: absolute
    top: 2px
    bottom: 8px
    left: 0
    padding: 3.5em 2em 2.5em
    width: 20em
    z-index: 1
    background: transparentize($black-ter, 0.3)
    backdrop-filter: blur(5px)
    pointer-events: none
    >>> li
      pointer-events: all

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
    display: flex
    justify-content: center

.generation-selector.is-finished
  >>> .flower-timeline .inner:after
    content: ''
    background: center center url('https://cdn0.iconfinder.com/data/icons/nature-and-environment-1/64/skeleton-pirate-crossbones-danger-deadly-skull-512.png') no-repeat
    background-size: contain
    width: 120px
    flex: 0 0 auto
</style>
