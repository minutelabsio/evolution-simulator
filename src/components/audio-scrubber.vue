<template lang="pug">
.scrubber(v-drag="onScrub")
  .nib(:style="{ transform: `translate(${progress}%)`}")
  .inner
    .progress-bar(:style="{ transform: `translate(${progress}%)`}")
</template>

<script>

export default {
  name: 'AudioScrubber'
  , props: {
    progress: Number
  }
  , components: {}
  , data: () => ({
  })
  , methods: {
    onScrub( e ){
      let x = e.layerX
      let progress = Math.round( x / this.$el.offsetWidth * 100 )
      progress = Math.min(Math.max(0, progress), 100)
      if ( e.first ){
        this.$emit('scrubstart')
      }
      if ( e.last ){
        this.$emit('scrubend')
      }
      if ( progress === this.progress ){ return }
      this.$emit('scrub', progress)
    }
  }
}
</script>

<style scoped lang="sass">
$pending-color: $grey-darker
$progress-color: $blue
.scrubber
  position: relative
  z-index: 1
  padding-top: 6px
  margin: -8px 0
  width: 100%
  height: 16px
  -webkit-touch-callout: none
  -webkit-user-select: none
  -khtml-user-select: none
  -moz-user-select: none
  -ms-user-select: none
  user-select: none
  -webkit-tap-highlight-color:  rgba(255, 255, 255, 0)
  cursor: pointer

  .nib
    position: absolute
    top: 0px
    left: 0px
    width: 100%
    z-index: 1
    transition: transform 0.1s ease
    &:after
      content: ''
      position: absolute
      left: -5px
      top: 3px
      display: block
      background: $progress-color
      width: 10px
      height: 10px
      border-radius: 10px


  .inner
    position: relative
    overflow: hidden
    height: 4px
    background: $pending-color

  .progress-bar
    position: relative
    right: 100%
    width: 100%
    height: 100%
    background: $progress-color
    // border-radius: 0 6px 6px 0
    transition: transform 0.1s ease
</style>
