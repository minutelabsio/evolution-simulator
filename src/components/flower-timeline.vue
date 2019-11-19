<template lang="pug">
.flower-timeline.scrollbars
  .inner
    .generation(
      v-for="(gen, index) in data"
      , @click="selected = index"
      , :class="{ selected: index === selected }"
    )
      FlowerChart(
        :data="gen"
        , :data-ranges="dataRanges"
        , :size="100"
        , :colors="colors"
        , :key="index"
      )
      .gen-label.no-select {{ index + 1 }}
</template>

<script>
import { scrollTo } from 'vue-scrollto'
import FlowerChart from '@/components/flower-chart'

const components = {
  FlowerChart
}

const computed = {
  selected: {
    get(){ return this.value }
    , set(val){ this.$emit('input', val) }
  }
}

const watch = {
  value(){
    this.$nextTick(() => {
      scrollTo('.selected', 500, {
        container: this.$el
        , x: true
        , y: false
        , offset: (el) => {
          return -(this.$el.offsetWidth - el.offsetWidth) / 2
        }
      })
    })
  }
}

export default {
  name: 'FlowerTimeline'
  , props: {
    data: Array
    , dataRanges: Object
    , colors: Object
    , value: Number // selected generation
  }
  , data: () => ({
  })
  , components
  , computed
  , watch
}
</script>

<style lang="sass" scoped>
.flower-timeline
  overflow: hidden
  width: 100%
  -webkit-overflow-scrolling: touch
  overflow-x: auto
  border: 1px solid transparentize($grey, 0.8)

  .inner
    display: flex
    // flex-wrap: nowrap
  .generation
    flex: 0 0 auto
    text-align: center
    cursor: pointer
    transition: background 0.15s ease
    &:hover
      background-color: rgba(255, 255, 255, 0.05)
    &:active,
    &.selected
      background-color: transparentize($blue, 0.8)
  .gen-label
    margin-bottom: 1em
  >>>
    circle.outer
      fill: $black-bis
</style>
