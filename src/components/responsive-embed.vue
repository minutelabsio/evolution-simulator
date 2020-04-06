<template lang="pug">
.responsive-embed-wrap(:style="{ maxWidth: maxWidth + 'px' }")
  .responsive-embed(:style="{ paddingTop: (1/ratio * 100) + '%' }")
    slot
</template>

<script>
export default {
  name: 'responsive-embed'
  , props: {
    ratio: {
      type: Number
      , default: 1
    }
    , maxWidth: Number
  }
  , data: () => ({
  })
  , components: {
  }
  , mounted(){
  }
  // , computed: {
  // }
  , methods: {
  }
}
</script>

<style lang="sass">
@import '@/styles/_variables.scss'
.responsive-embed-wrap
  margin: auto
.responsive-embed
  position: relative
  > *
    position: absolute
    top: 0
    left: 0
    right: 0
    bottom: 0
    width: 100%
    height: 100%
</style>
