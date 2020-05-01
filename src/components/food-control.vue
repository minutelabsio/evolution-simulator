<template lang="pug">
.food-control
  .col
    label Gen.:
    label Food:
  .inner.scrollbars
    .col(v-for="(entry, index) in data")
      b-input(type="text", v-model.number="entry[0]", :disabled="index === 0", @blur="commit")
      b-input(type="text", v-model.number="entry[1]", :disabled="index === 0", @blur="commit")
  .col
    .mid
      b-tooltip(label="Add a control point", position="is-top")
        b-button.btn-dark(@click="addCol")
          b-icon(icon="plus")
      b-tooltip(label="Clear", position="is-top")
        b-button.btn-dark(@click="data = []; commit()")
          b-icon(icon="delete")
</template>

<script>
import _cloneDeep from 'lodash/cloneDeep'
import { mapGetters } from 'vuex'

const components = {
}

const computed = {
  ...mapGetters('simulation', {
    config: 'config'
  })
}

const watch = {
  'config.preset.food_vector': {
    handler(f){
      this.data = _cloneDeep(f)
    }
    , immediate: true
  }
}

const methods = {
  addCol(){
    let last = this.data[this.data.length - 1]
    this.data.push([last[0]+1, 0])
    this.commit()
  }
  , commit(){
    this.data = this.data.filter(d => {
      return Number.isFinite(d[0]) && Number.isFinite(d[1]) && d[0] > 0
    })
    this.data.sort((a, b) => a[0] - b[0])
    this.data.unshift([0, this.config.food_per_generation])

    this.$store.dispatch('simulation/setConfig', { preset: {
      ...this.config.preset
      , food_vector: _cloneDeep(this.data)
    } })
  }
}

export default {
  name: 'FoodControl'
  , props: {
  }
  , data: () => ({
    data: [[1, 50]]
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
  flex-direction: row
.inner
  display: flex
  flex-direction: row
  overflow-x: auto
  max-width: 720px
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
</style>
