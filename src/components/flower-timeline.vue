<template lang="pug">
.flower-timeline.scrollbars(@wheel="onScroll")
  .inner
    slot(name="before")
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
        , :top-petal="topPetal"
        , @select="onFlowerSelect($event, index)"
      )
      .gen-label.no-select {{ index + 1 }}
    slot(name="after")
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
    this.focusSelected()
  }
  , data(data, oldValue){
    if (!oldValue || !oldValue.length){
      this.focusSelected()
    }
  }
}

const methods = {
  onScroll( e ){
    // e.preventDefault()
    const el = this.$el
    if ( e.deltaY ){
      el.scrollLeft += e.deltaY
    }
  }
  , onFlowerSelect( selected, generation ){
    this.$emit('dataSelect', {
      generation
      , selected
    })
  }
  , focusSelected(){
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
    , topPetal: Number
    , value: Number // selected generation
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
.flower-timeline
  overflow: hidden
  width: 100%
  min-height: 148px
  -webkit-overflow-scrolling: touch
  overflow-x: auto
  // border: 1px solid transparentize($grey, 0.8)

  .inner
    display: flex
    min-height: 146px
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
      transition: none
      background-color: transparentize($primary, 0.8)
  .gen-label
    margin-bottom: 1em
  >>>
    circle.outer
      fill: $black-bis
</style>
