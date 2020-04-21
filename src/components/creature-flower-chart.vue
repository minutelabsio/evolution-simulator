<template lang="pug">
.creature-flower-chart
  .bg
    FlowerChart(
      :data="traitData"
      , :data-ranges="generationRanges"
      , :size="size"
      , :colors="colors"
      , :show-center="false"
      , :show-values="false"
      , :values-outside="true"
    )
  label
    span='Current Creature '
    b-tooltip(label="Flower diagram is sized relative to averages in the current generation", position="is-left", multilined)
      b-icon(icon="help-circle", size="is-small")
</template>

<script>
import { mapGetters } from 'vuex'
import _find from 'lodash/find'
import _get from 'lodash/get'
import FlowerChart from '@/components/flower-chart'

const components = {
  FlowerChart
}

const computed = {
  colors(){
    return {
      petals: this.traitColors
    }
  }
  , creature(){
    let g = this.getCurrentGeneration()
    return _find(g.creatures, { id: this.id })
  }
  , traitData(){
    return {
      center: 0
      , petals: this.traits.map(t => _get(this.creature[t], '0', this.creature[t]))
    }
  }
  , generationRanges(){
    let s = this.generationStats
    return {
      center: [0, 1]
      , petals: this.traits.map(t => [s[t].min, s[t].max])
    }
  }
  , generationStats(){
    if (!this.statistics){ return {} }
    return this.statistics.generation_statistics[this.currentGenerationIndex]
  }
  , ...mapGetters('simulation', {
    'statistics': 'statistics'
    , 'getCurrentGeneration': 'getCurrentGeneration'
    , 'currentGenerationIndex': 'currentGenerationIndex'
    , 'traits': 'traits'
    , 'traitColors': 'traitColors'
  })
}

const watch = {
}

const methods = {
}

export default {
  name: 'CreatureFlowerChart'
  , props: {
    id: String
    , size: {
      type: Number
      , default: 200
    }
  }
  , data: () => ({
  })
  , components
  , computed
  , watch
  , methods
}
</script>

<style lang="sass" scoped>
.creature-flower-chart
  color: lighten($grey, 10)
  text-align: center
  label
    // text-shadow: 0 0 4px rgba(255, 255, 255, 0.5)
    font-family: $family-monospace
  .bg
    border-radius: 50%
    >>> svg
      font-size: .2px
      circle
        fill: transparentize($background, 0.1)
</style>
