<template lang="pug">
.flower-timeline.scrollbars
  .inner
    .generation(v-for="(gen, index) in data")
      FlowerChart(
        :data="gen"
        , :data-ranges="dataRanges"
        , :size="100"
        , :colors="colors"
      )
      .gen-label {{ index + 1 }}
</template>

<script>
import FlowerChart from '@/components/flower-chart'

const components = {
  FlowerChart
}

export default {
  name: 'FlowerTimeline'
  , props: {
    data: Array
    , dataRanges: Object
    , colors: Object
  }
  , data: () => ({
  })
  , components
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
  .gen-label
    margin-bottom: 1em
</style>
