<template lang="pug">
.simulation-statistics.scrollbars
  //- pre {{ speedData | json:2 }}
  b-collapse
    h2.heading.plot-title(slot="trigger", slot-scope="props", :style="{ color: populationColor }")
      b-icon(:icon="props.open ? 'minus-box' : 'plus-box'")
      | &nbsp;
      span.name Population
    TraitPlot(:data="populationData", label="Population", :color="populationColor")
  b-collapse(v-for="trait in traits", :key="trait")
    h2.heading.plot-title(slot="trigger", slot-scope="props", :style="{ color: titleColors[trait] }")
      b-icon(:icon="props.open ? 'minus-box' : 'plus-box'")
      | &nbsp;
      span.name {{ trait | startCase }}
    TraitPlot(:data="traitData[trait]", :label="trait | startCase", :color="traitColors[trait]")
</template>

<script>
import { mapGetters } from 'vuex'
import _mapValues from 'lodash/mapValues'
import chroma from 'chroma-js'
import CreatureTraits from '@/config/creature-traits'
import TraitPlot from '@/components/trait-plot'

export default {
  name: 'SimulationStatistics'
  , props: {
  }
  , data: () => ({
    traits: CreatureTraits.traits
    , traitColors: CreatureTraits.hashed
    , populationColor: CreatureTraits.populationColor
    , titleColors: _mapValues(CreatureTraits.hashed, c => chroma(c).desaturate(1).css())
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
    , traitData(){
      if (!this.statistics){ return {} }
      let data = {}
      this.traits.forEach(t => {
        data[t] = this.statistics.generation_statistics.map(d => [d[t].mean, d[t].deviation])
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
  }
}
</script>

<style lang="sass" scoped>
.simulation-statistics
  padding: 6em 2em 0
  height: 100%
  overflow: scroll
.plot-title
  margin-top: 1em
  text-align: center
  .icon
    position: relative
    top: 2px
  .name
    font-size: 24px
</style>
