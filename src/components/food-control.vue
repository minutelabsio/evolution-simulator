<template lang="pug">
.food-control
  .cols
    .col
      label Gen.:
      label Food:
    .inner.scrollbars
      .col(v-for="(entry, index) in data")
        b-input(type="text", v-model.number="entry[0]", :disabled="index === 0", @blur="commit")
        b-input(type="text", v-model.number="entry[1]", :disabled="index === 0", @blur="commit")
    .col.btns
      .mid
        b-tooltip(label="Add a control point", position="is-top")
          b-button.btn-dark(@click="addCol")
            b-icon(icon="plus")
        b-tooltip(label="Clear", position="is-top")
          b-button.btn-dark(@click="data = []; commit()")
            b-icon(icon="delete")
  .plot
    FoodPlot(:data="foodPlotData", :styles="{ height: '160px' }")
</template>

<script>
// import _cloneDeep from 'lodash/cloneDeep'
import _times from 'lodash/times'
import _flatten from 'lodash/flatten'
import { mapGetters } from 'vuex'
import FoodPlot from '@/components/food-plot'
import sougy from '@/config/sougy-colors'
import interpolator from '@/lib/interpolator'

const foodColor = sougy.green

const components = {
  FoodPlot
}

const computed = {
  ...mapGetters('simulation', {
    config: 'config'
  })
  , foodPlotData(){
    let generate = interpolator(_flatten(this.data))
    let data = _times(this.config.max_generations, (n) => [n, Math.round(generate(n))])
    return [{ label: 'Available Food', color: foodColor, data }]
  }
}

const watch = {
  'config.food_per_generation': {
    handler(data){
      this.data = data.map(([g, f]) => [g + 1, f])
    }
    , immediate: true
  }
}

const methods = {
  addCol(){
    let last = this.data[this.data.length - 1]
    this.data.push([last[0]+1, last[1]])
    this.commit()
  }
  , commit(){
    this.data = this.data.filter(d => {
      return Number.isFinite(d[0]) && Number.isFinite(d[1]) && d[0] > 1
    })
    this.data.sort((a, b) => a[0] - b[0])
    this.data.unshift([1, this.config.food_per_generation[0][1]])

    this.$store.dispatch('simulation/setConfig', {
      food_per_generation: this.data.map(([g, f]) => [g - 1, f])
    })
  }
}

export default {
  name: 'FoodControl'
  , props: {
  }
  , data: () => ({
    data: [[1, 50]]
    , foodColor
  })
  , components
  , computed
  , watch
  , methods
}
</script>

<style lang="sass" scoped>
.food-control
  display: flex
  flex-direction: column
  align-items: flex-start
.plot
  position: relative
  max-height: 10rem
  line-height: 0
.cols
  display: flex
  flex-direction: row
  margin-left: 0.5rem
.inner
  display: flex
  flex-direction: row
  overflow-x: auto
  max-width: 360px
  @media screen and (max-width: $tablet)
    max-width: 70vw
.col
  position: relative
  display: flex
  flex-direction: column
  justify-content: space-evenly
  min-width: 4rem
  max-width: 4rem
  >>> .control .input
    position: relative
    text-align: center
    border-radius: 0
    &:not(:disabled):focus
      z-index: 1
    &:not(:disabled):hover
      z-index: 2
  >>> .control:not(:first-child) .input
    top: -1px
.col:not(:first-child)
  margin-left: -1px
.col.btns
  margin-left: 0.5rem
</style>
