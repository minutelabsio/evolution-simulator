<template lang="pug">
.simulation-statistics.scrollbars
  .field.is-grouped.is-grouped.multiline
    .tags
      .tag.is-large.clickable(v-for="plot in hiddenPlots", :style="{ color: plot.titleColor }", @click="showPlot(plot)")
        b-icon(icon="plus-box")
        span {{ plot.name | startCase }}
  .columns.is-multiline
    .column(v-for="plot in plots", :key="plot.name")
      h2.heading.plot-title.clickable(:style="{ color: plot.titleColor }", @click="hidePlot(plot)")
        b-icon(icon="minus-box")
        | &nbsp;
        span.name {{ plot.name | startCase }}
      TraitPlot(:data="plot.data", :label="plot.name | startCase", :color="plot.color")
</template>

<script>
import { mapGetters } from 'vuex'
import _mapValues from 'lodash/mapValues'
import _findIndex from 'lodash/findIndex'
import chroma from 'chroma-js'
import CreatureTraits from '@/config/creature-traits'
import TraitPlot from '@/components/trait-plot'

const titleColors = _mapValues(CreatureTraits.hashed, c => chroma(c).desaturate(1).css())

export default {
  name: 'SimulationStatistics'
  , props: {
  }
  , data: () => ({
    traits: CreatureTraits.traits
    , traitColors: CreatureTraits.hashed
    , populationColor: CreatureTraits.populationColor
    , titleColors
    , hiddenPlots: []
  })
  , components: {
    TraitPlot
  }
  , created(){
  }
  , deactivated(){
  }
  , beforeDestroy(){
  }
  , mounted(){
  }
  , computed: {
    ...mapGetters('simulation', {
      isLoading: 'isLoading'
      , generationIndex: 'currentGenerationIndex'
      , statistics: 'statistics'
    })
    , plots(){
      return [{
        color: CreatureTraits.populationColor
        , titleColor: CreatureTraits.populationColor
        , data: this.populationData
        , name: 'population'
      }].concat(
        CreatureTraits.traits.map(t => ({
          color: CreatureTraits.hashed[t]
          , titleColor: titleColors[t]
          , data: this.traitData[t]
          , name: t
        }))
      ).filter(v => _findIndex(this.hiddenPlots, ['name', v.name]) < 0)
    }
    , traitData(){
      if (!this.statistics){ return {} }
      let data = {}
      this.traits.forEach(t => {
        data[t] = this.statistics.generation_statistics.map(d => d[t])
      })
      return data
    }
    , populationData(){
      if (!this.statistics){ return [] }
      return this.statistics.generation_statistics.map(d => d.population)
    }
  }
  , watch: {
  }
  , methods: {
    showPlot(plot){
      let index = this.hiddenPlots.indexOf(plot)
      this.hiddenPlots.splice(index, 1)
    }
    , hidePlot(plot){
      this.hiddenPlots.push(plot)
    }
  }
}
</script>

<style lang="sass" scoped>
.simulation-statistics
  padding: 6em 2em 0
  overflow-y: scroll
.plot-title
  margin-top: 1em
  text-align: center
  white-space: nowrap
  .icon
    position: relative
    top: 2px
  .name
    font-size: 24px
</style>
