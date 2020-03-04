<template lang="pug">
.playground
  SimulationConfig.cfg.scrollbars(v-if="showConfig")
  .top-bar
    .logo
      img(src="@/assets/logo-dark.png")
    router-link(:to="{ params: $route.params, name: 'viewscreen' }", exact)
      span World
    router-link(:to="{ params: $route.params, name: 'stats' }", exact)
      span Stats
    router-link(:to="{ params: $route.params, query: { cfg: '1' } }", exact)
      span Settings

  .upper
    keep-alive(includes="ViewScreen")
      router-view

  .bottom-drawer
    b-loading.loading-overlay(:is-full-page="false", :active="isLoading")
    Drawer.legend-drawer(direction="right", :start-open="legendStartsOpen")
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
          b-button.btn-dark(@click="continueSimulation", v-if="!isLoading") Load More
          .flower-timeline-spacer
</template>

<script>
// import _throttle from 'lodash/throttle'
import { mapGetters } from 'vuex'
import CreatureTraits from '@/config/creature-traits'
import FlowerChart from '@/components/flower-chart'
import FlowerTimeline from '@/components/flower-timeline'
import Drawer from '@/components/drawer'
import Legend from '@/components/legend'
import SimulationConfig from '@/components/simulation-config'

const creatureTraits = CreatureTraits.traits

export default {
  name: 'Simulation'
  , props: {
    showConfig: Boolean
  }
  , components: {
    FlowerChart
    , FlowerTimeline
    , Legend
    , Drawer
    , SimulationConfig
  }
  , data: () => ({
    toolbar: true

    , flowerColors: {
      center: CreatureTraits.populationColor
      , petals: CreatureTraits.colors
    }
    , topPetal: 0
    , legendStartsOpen: window.innerWidth > 620
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
  min-height: 0
.cfg
  position: absolute
  top: 0
  left: 0
  right: 0
  bottom: 0
  z-index: 4
  background: rgba(0, 0, 0, 0.35)
  backdrop-filter: blur(5px)
  padding-top: 6rem
  overflow: auto

.top-bar
  position: absolute
  top: 1.25em
  left: 1.75em
  pointer-events: none
  z-index: 10
  height: 36px
  // opacity: 0.7
  display: flex
  align-items: center
  // background: #333
  // border-radius: 0 0 3px 0
  backdrop-filter: blur(2px)
  > *
    pointer-events: all
    margin-right: 1rem
    text-shadow: 0 0 3px #333
  .logo
    margin-right: 0.75rem
    overflow: hidden
    height: 30px

    img
      width: 40px
  a
    color: lighten($grey, 10)
    font-size: 24px
    transition: color .15s ease
    &.router-link-active,
    &:hover
      color: $grey-lighter
.config-link
  position: absolute
  top: 1.5rem
  right: 1.75rem
  display: flex
  align-items: center
  z-index: 3
.upper
  position: relative
  z-index: 2
  flex-grow: 1
  min-height: 0
  // overflow: hidden
  background: #333333
  // max-height: calc(100vh - 214px)
.bottom-drawer
  position: relative
  background: $black-ter
  border-top: 1px solid $black
  // min-height: 213px
  .legend-drawer
    // position: absolute
    top: 2px
    bottom: 8px
    left: 0
    display: flex
    box-shadow: none
    border: none
    background: transparentize(black, 0.3)
    backdrop-filter: blur(5px)
    >>> .collapse-bar
      border-right: 1px solid $grey-darker
      border-radius: 0
  .legend
    align-self: center
    padding: 8px 2em 0
    width: 18em
    z-index: 1

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
.loading-overlay
  z-index: 2
  background: rgba(0, 0, 0, .5)
.generation-selector.is-finished
  >>> .flower-timeline .inner:after
    content: ''
    background: center center url('https://cdn0.iconfinder.com/data/icons/nature-and-environment-1/64/skeleton-pirate-crossbones-danger-deadly-skull-512.png') no-repeat
    background-size: contain
    width: 120px
    flex: 0 0 auto
</style>
