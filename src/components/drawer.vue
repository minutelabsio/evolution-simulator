<template lang="pug">
.drawer(:class="{ collapsed: !isOpen }")
  button.collapse-bar(@click="isOpen = !isOpen")
    b-icon(:icon="isOpen ? 'menu-right' : 'menu-left'")
  slot
</template>

<script>
const components = {

}

const computed = {
  isOpen: {
    get(){
      if (this.open === undefined){
        return this.openState
      } else {
        return this.open
      }
    }
    , set(v){
      this.openState = v
      this.$emit('update:open', v)
    }
  }
}

const watch = {
}

const methods = {
}

export default {
  name: 'Drawer'
  , props: {
    open: {
      type: Boolean
      , default: undefined
    }
    , direction: {
      type: String
      // TODO....
      , validator: v => (['right', 'top', 'left', 'bottm'].indexOf(v) > -1)
      , default: 'right'
    }
  }
  , data: () => ({
    openState: false
  })
  , components
  , computed
  , watch
  , methods
}
</script>

<style lang="sass" scoped>
.drawer
  position: absolute
  right: 0
  z-index: 1
  min-width: 60px
  padding: 1.5em
  padding-left: 2em
  border-radius: 4px 0 0 4px
  border: 1px solid darken($grey, 10)
  border-right: none
  background: rgba(0, 0, 0, 0.3)
  backdrop-filter: blur(3px)
  box-shadow: 2px 2px 3px rgba(0, 0, 0, 0.3)

  transition: transform 0.4s ease

  &.collapsed
    transform: translate3d(calc(100% - 20px), 0, 0)

  .collapse-bar
    position: absolute
    top: 0
    left: 0
    bottom: 0
    width: 20px
    border: none
    overflow: hidden
    background: transparentize($background, 0.1)
    padding: 0
    border-radius: 4px 0 0 4px
    color: $text
    cursor: pointer
    outline: none
    transition: background 0.15s ease
    &:hover
      background: $background
    &:active
      background: lighten($background, 2)
    .icon
      width: 100%
</style>
