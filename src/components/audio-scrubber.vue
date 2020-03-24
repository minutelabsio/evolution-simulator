<script>
export default {
  name: 'AudioScrubber'
  , functional: true
  , props: {
    progress: Number
  }
  , render(h, ctx){
    let currentProgress = ctx.props.progress
    let style = { transform: `translate(${currentProgress}%)` }
    const $emit = (name, ...args) => {
      let cb = ctx.listeners[name]
      if (cb){
        cb.apply(null, args)
      }
    }
    return h('div',
      {
        class: 'scrubber'
        , directives: [
          {
            name: 'drag'
            , value: ( e ) => {
              let el = e.el
              let x = e.layerX
              let progress = x / el.offsetWidth * 100
              progress = Math.min(Math.max(0, progress), 100)
              if ( e.first ){
                $emit('scrubstart')
              }
              if ( e.last ){
                $emit('scrubend')
              }
              if ( progress === currentProgress ){ return }
              $emit('scrub', progress)
            }
          }
        ]
      }
      , [
        h('div', { class: 'nib', style })
        , h('div', { class: 'inner' }, [
          h('div', { class: 'progress-bar', style })
        ])
      ]
    )
  }
}
</script>

<style scoped lang="sass">
$pending-color: $grey-darker
$progress-color: lighten($primary, 8)
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
    will-change: transform
    // transition: transform 0.01s ease-in
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
    will-change: transform
    // border-radius: 0 6px 6px 0
    // transition: transform 0.01s ease-in
</style>
