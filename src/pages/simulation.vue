<template lang="pug">
.playground
  SimulationConfig.cfg.scrollbars(v-if="showConfig")
  transition(name="slide-down", appear)
    .top-bar(v-if="!showIntro")
      a(href="https://minutelabs.io", target="_blank").logo
        img(src="@/assets/logo-dark.png")
      router-link(:to="{ name: 'about' }")
        span About
      router-link(:to="{ params: $route.params, name: 'viewscreen' }")
        span World
      router-link(:to="{ params: $route.params, name: 'stats' }")
        span Trends

  transition(name="slide-down", appear)
    .center-bar(v-if="!hideSettings && !showIntro || tourStepNumber === 6")
      router-link.button.is-primary.is-rounded(v-if="!showConfig", :to="{ query: { cfg: $route.query.cfg ? undefined : '1' } }", append, exact)
        span Settings

  .upper
    keep-alive(includes="ViewScreen")
      router-view(ref="view")

  transition(name="bottom-drawer-slide", appear, @after-appear="fixLayout", @after-enter="fixLayout", @after-leave="fixLayout")
    .bottom-drawer(v-if="!hideControls && !showIntro")
      .floating-more-btn
        b-field
          p.control
            b-tooltip(label="Run the simulation with a new random seed", position="is-left")
              b-button.is-outlined.btn-dark(@click="shuffleSimulation", :loading="isContinuing", icon-right="dice-5")
          p.control
            b-tooltip(label="More Generations...", position="is-left")
              b-button.is-outlined.btn-dark(@click="continueSimulation", :loading="isContinuing", icon-right="layers-plus")
          p.control
            b-tooltip(label="Toggle Drawer", position="is-left")
              b-button.is-outlined.btn-dark(@click="showBottomDrawer = !showBottomDrawer", :icon-right="showBottomDrawer ? 'menu-down' : 'menu-up'")
      transition(name="collapse", @after-enter="fixLayout", @after-leave="fixLayout")
        .bottom-drawer-content(v-if="showBottomDrawer")
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
                .more-btn
                  b-button.btn-dark(@click="continueSimulation", :loading="isContinuing", size="is-large", icon-right="layers-plus")
                .flower-timeline-spacer
              template(v-else, #after)
                .extinct-status
                  img(src="@/assets/status-icons/extinction.svg", width="70")
                .flower-timeline-spacer
</template>

<script>
// import _throttle from 'lodash/throttle'
import { mapGetters } from 'vuex'
import traitColors from '@/config/trait-colors'
import FlowerChart from '@/components/flower-chart'
import FlowerTimeline from '@/components/flower-timeline'
import Drawer from '@/components/drawer'
import Legend from '@/components/legend'
import SimulationConfig from '@/components/simulation-config'

export default {
  name: 'Simulation'
  , props: {
    showConfig: Boolean
    , showIntro: Number
    , hideControls: Boolean
    , hideSettings: Boolean
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
    , showBottomDrawer: true

    , topPetal: 0
    , legendStartsOpen: window.innerWidth > 620
  })
  , created(){
  }
  , mounted(){
    this.$nextTick(() => {
      let idx = this.generationIndex
      if (idx){
        let max_generations = Math.max(idx + 1, this.config.max_generations)
        this.$store.dispatch('simulation/setConfig', { max_generations })
      }

      if (!this.generation){
        this.run(false)
      }
    })
  }
  , watch: {
  }
  , computed: {
    tourStepNumber(){
      return this.$route.query.intro | 0
    }
    , flowerColors(){
      return {
        center: traitColors.population
        , petals: this.traitColors
      }
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
        , petals: this.traits.map(k => g[k].mean)
      }
    }
    , flowerTimelineData(){
      if (!this.stats){ return [] }
      let stats = this.generationStats
      return stats.map(v => ({
        center: v.population
        , petals: this.traits.map(k => v[k].mean)
      }))
    }
    , flowerRanges(){
      if ( !this.stats ){ return {} }
      let { population } = this.stats
      return {
        center: [population.min, population.max]
        , petals: this.traits.map(k => this.stats[k]).map(t => [t.min, t.max])
      }
    }
    , flowerLegend(){
      return Object.values(this.traits)
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
      , isContinuing: 'isContinuing'
      , getCurrentGeneration: 'getCurrentGeneration'
      , generationIndex: 'currentGenerationIndex'
      , config: 'config'
      , stats: 'statistics'
      , traits: 'traits'
      , traitColors: 'traitColors'
    })
  }
  , methods: {
    run(fresh){
      return this.$store.dispatch('simulation/run', fresh)
    }
    , continueSimulation(){
      return this.$store.dispatch('simulation/continue')
    }
    , shuffleSimulation(){
      let seed = (Math.random() * 1000) | 0
      this.$store.dispatch('simulation/setConfig', { seed })
      this.run()
    }
    , loadGeneration(v){
      this.$store.dispatch('simulation/loadGeneration', v)
    }
    , propertySelect(index){
      if ( index !== undefined && index < this.traits.length ){
        this.topPetal = index
      }
    }
    , fixLayout(){
      let view = this.$refs.view
      if (view.fixLayout){
        view.fixLayout()
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
  background: rgba(0, 0, 0, 0.65)
  backdrop-filter: blur(5px)
  padding-top: 6rem
  overflow: auto

.top-bar,
.center-bar
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
  // backdrop-filter: blur(2px)
  > *
    pointer-events: all
    margin-right: 1rem
    text-shadow: 0 0 1px $grey-darker

.top-bar
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

.center-bar
  left: 50%
  > *
    transform: translateX(-50%)
  .button
    text-transform: lowercase
    font-size: 1rem
    margin-right: 0

  @media screen and (max-width: $tablet)
    left: 2rem
    top: 4.5rem
    > *
      transform: none

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
  background: $grey-darker
  display: flex
  align-items: stretch
  > *
    flex: 1

  // max-height: calc(100vh - 214px)
.bottom-drawer-content
  height: 255px
  &.collapse-enter-active, &.collapse-leave-active
    transition: height .3s ease
    .legend-drawer
      transition: transform .1s ease
    .generation-selector >>> .flower-timeline:after
      content: ''
  &.collapse-enter-active
    .legend-drawer
      transition: transform .1s .2s ease
  &.collapse-enter, &.collapse-leave-to
    height: 0
    .legend-drawer
      transform: translate(-100%, 0)
.bottom-drawer
  position: relative
  background: $black-ter
  border-top: 1px solid $black
  transition: height .3s ease
  min-height: 3.25rem
  flex-shrink: 0
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
    padding: 1rem 2rem 1rem 1rem
    >>> .collapse-bar
      border-right: 1px solid $grey-darker
      border-radius: 0
  .legend
    flex-direction: column
    align-self: center
    padding: 0 0em 0
    // width: 13em
    z-index: 1

    pointer-events: none
    >>> li
      pointer-events: all
  .floating-more-btn
    position: absolute
    top: 0.5rem
    right: 0.5rem
    display: flex
    align-items: center
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
    padding-top: 46px

    padding-bottom: 56px
    &:after
      content: 'Generations'
      position: absolute
      bottom: 20px
      left: 50vw
      transform: translateX(-50%)
      font-family: $family-monospace
      font-size: 1rem
      text-transform: uppercase
      text-shadow: 0 1px 3px rgba(0, 0, 0, 1)

    .generation.selected
      position: relative
      &:before
        content: ''
        pointer-events: none
        position: absolute
        top: -80px
        left: 50%
        margin-left: -50px
        border: 50px solid transparent
        border-color: transparent transparent transparentize($primary, 0.8) transparent
        border-width: 50px 50px 30px 50px
  .flower-timeline-spacer
    flex: 0 0 auto
    width: calc(50vw - 50px)
    display: flex
    justify-content: center
.loading-overlay
  z-index: 2
  background: rgba(0, 0, 0, .5)
.more-btn
  display: flex
  align-items: center
  justify-content: center
  min-width: 100px
  .button
    margin-top: -60px
.extinct-status
  flex: 0 0 auto
  width: 92px
  height: 92px
  margin: 4px
  margin-right: 2rem
  border: 1px solid $grey-darker
  border-radius: 50%
  display: flex
  align-items: center
  justify-content: center
.bottom-drawer-slide-enter-active, .bottom-drawer-slide-leave-active
  transition: margin-bottom 0.3s ease
.bottom-drawer-slide-enter, .bottom-drawer-slide-leave-to
  margin-bottom: -256px
</style>
