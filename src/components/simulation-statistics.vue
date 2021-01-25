<template lang="pug">
.simulation-statistics.scrollbars
  .columns
    .column.is-one-half
      b-field(label="Filter By Species")
        b-select(v-model="statsSpeciesFilter")
          option(v-for="(p, i) in speciesFilterList", :value="i", :key="p[1]") {{ p[1] }}
    .column
      b-field.generation-controls
        p.control
          b-tooltip(label="Download the data as a CSV", position="is-left")
            b-button.btn-dark(@click="downloadCSV", :loading="isDownloading", icon-left="download") Download
        p.control
          b-tooltip(label="Run the simulation again", position="is-left")
            b-button.btn-dark(@click="restart()", :loading="isContinuing", icon-right="reload")
        p.control
          b-tooltip(label="Run the simulation with a new random seed", position="is-left")
            b-button.btn-dark(@click="shuffleSimulation()", :loading="isContinuing", icon-right="dice-5")
        p.control
          b-tooltip(label="See more generations", position="is-left")
            b-button.btn-dark(@click="keepGoing()", :loading="isContinuing", icon-right="layers-plus")
  .field.is-grouped.is-grouped.multiline
    .tags
      .tag.is-large.clickable(v-for="plot in hiddenPlots", :style="{ color: plot.titleColor }", @click="showPlot(plot)")
        b-icon(icon="plus-box")
        span {{ plot.label | startCase }}

  .columns
    .column
      h2.heading.plot-title
        span.name Overview
      OverviewPlot(:data="plots", label="Overview")
  .columns
    .column
      h2.heading.plot-title
        span.name Births and Deaths
      BirthDeathPlot(:data="birthsDeathsData", :stacked="true", @click="loadGeneration")
    .column
      h2.heading.plot-title
        span.name Food
      FoodPlot(:data="foodData", :maxFood="maxFood", @click="loadGeneration")
  .columns.is-multiline
    .column.is-half-tablet.is-one-third-fullhd(v-for="plot in plots", :key="plot.label")
      h2.heading.plot-title.clickable(:style="{ color: plot.titleColor }", @click="hidePlot(plot)")
        b-icon(icon="minus-box")
        | &nbsp;
        span.name {{ plot.label | startCase }}
      TraitPlot(:data="plot.data", :label="plot.label | startCase", :color="plot.color", @click="loadGeneration")
  a.is-hidden(ref="downloadEl")
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import _mapValues from 'lodash/mapValues'
import _findIndex from 'lodash/findIndex'
import chroma from 'chroma-js'
import sougy from '@/config/sougy-colors'
import traitColors from '@/config/trait-colors'
import TraitPlot from '@/components/trait-plot'
import OverviewPlot from '@/components/overview-plot'
import FoodPlot from '@/components/food-plot'
import BirthDeathPlot from '@/components/birth-death-plot'

const titleColors = _mapValues(traitColors, c => chroma(c).desaturate(1).css())

export default {
  name: 'SimulationStatistics'
  , props: {
  }
  , data: () => ({
    traitColors
    , populationColor: traitColors.population
    , titleColors
    , hiddenPlots: []
    , isDownloading: false
  })
  , components: {
    TraitPlot
    , FoodPlot
    , OverviewPlot
    , BirthDeathPlot
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
      generationIndex: 'currentGenerationIndex'
      , statistics: 'statistics'
      , traits: 'traits'
      , isLoading: 'isLoading'
      , isContinuing: 'isContinuing'
      , speciesFilterList: 'speciesFilterList'
    })
    , statsSpeciesFilter: {
      get(){
        return this.$store.state.simulation.statsSpeciesFilter
      }
      , set(v){
        this.$store.dispatch('simulation/setStatsFilter', v)
      }
    }
    , plots(){
      return [{
        color: traitColors.population
        , titleColor: traitColors.population
        , data: this.populationData
        , label: 'population'
        , yAxisID: 'population'
      }].concat(
        this.traits.map(t => ({
          color: traitColors[t]
          , titleColor: titleColors[t]
          , data: this.traitData[t]
          , label: t
        }))
      ).filter(v => _findIndex(this.hiddenPlots, ['label', v.label]) < 0)
    }
    , birthsDeathsData(){
      if (!this.statistics){ return [] }
      const gs = this.statistics.generation_statistics
      return [{
        color: sougy.ivory
        , backgroundColor: chroma(sougy.ivory).alpha(0.5).css()
        , fill: 'origin'
        , label: 'Net Change'
        , type: 'line'
        , data: gs.map((g, i) => [i + 1, g.births - g.deaths])
      }, {
        color: sougy.green
        , backgroundColor: chroma(sougy.green).alpha(0.5).css()
        , fill: 'origin'
        , label: 'Births'
        , stack: 'stack'
        , data: gs.map((g, i) => [i + 1, g.births])
      }, {
        color: sougy.red
        , backgroundColor: chroma(sougy.red).alpha(0.5).css()
        , fill: 'origin'
        , label: 'Deaths'
        , stack: 'stack'
        , data: gs.map((g, i) => [i + 1, -g.deaths])
      }]
    }
    , foodData(){
      if (!this.statistics){ return [] }
      const gs = this.statistics.generation_statistics
      return [{
        color: sougy.ivory
        , backgroundColor: chroma(sougy.ivory).alpha(0.2).css()
        , fill: '+1'
        , label: 'Food Available'
        , data: gs.map((g, i) => [i + 1, g.food_balls_available])
      }, {
        color: sougy.green
        , backgroundColor: chroma(sougy.green).alpha(0.5).css()
        , fill: 'origin'
        , label: 'Food Eaten'
        , data: gs.map((g, i) => [i + 1, g.food_balls_eaten])
      }, {
        color: sougy.red
        , backgroundColor: chroma(sougy.red).alpha(0.5).css()
        , fill: 'origin'
        , label: 'Creatures Eaten'
        , data: gs.map((g, i) => [i + 1, g.creatures_eaten])
      }]
    }
    , maxFood(){
      if (!this.statistics){ return 0 }
      return this.statistics.generation_statistics.reduce((a, g) =>
        Math.max(a, g.food_balls_available)
      , 0)
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
    async downloadCSV(){
      this.isDownloading = true
      let csvContent = await this.$store.dispatch('simulation/getCSV')
      let date = (new Date()).toISOString()
      let speciesFilterLabel = this.speciesFilterList[this.statsSpeciesFilter][1]
      let link = this.$refs.downloadEl
      let blob = new Blob([csvContent], {type: 'text/csv'})
      let href = window.URL.createObjectURL(blob)
      link.setAttribute('href', href)
      link.setAttribute('download', `EvolutionSimulatorData_${speciesFilterLabel}_${date}.csv`)
      link.click()
      this.isDownloading = false
    }
    , showPlot(plot){
      let index = this.hiddenPlots.indexOf(plot)
      this.hiddenPlots.splice(index, 1)
    }
    , hidePlot(plot){
      this.hiddenPlots.push(plot)
    }
    , shuffleSimulation(){
      let seed = (Math.random() * 1000) | 0
      this.$store.dispatch('simulation/setConfig', { seed })
      this.restart()
    }
    , loadGeneration(generationIndex){
      let route = this.$route
      let params = route ? route.params : {}
      let query = route ? route.query : {}
      this.$router.push({ name: 'viewscreen', params: { ...params, generationIndex }, query })
    }
    , ...mapActions('simulation', {
      restart: 'run'
      , keepGoing: 'continue'
    })
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
.field.generation-controls
  justify-content: flex-end
</style>
