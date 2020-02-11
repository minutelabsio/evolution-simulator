<template lang="pug">
.drawer(:class="{ collapsed: !isOpen, ['direction-' + direction]: true }")
  button.collapse-bar(@click="isOpen = !isOpen")
    b-icon(:icon="isOpen ? closeIcon : openIcon")
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
  , closeIcon(){
    switch (this.direction) {
      case 'right':
        return 'menu-left'
      case 'left':
        return 'menu-right'
      case 'up':
        return 'menu-down'
      case 'down':
        return 'menu-up'
    }
  }
  , openIcon(){
    switch (this.direction) {
      case 'right':
        return 'menu-right'
      case 'left':
        return 'menu-left'
      case 'up':
        return 'menu-up'
      case 'down':
        return 'menu-down'
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
    , startOpen: {
      type: Boolean
    }
    , direction: {
      type: String
      // TODO....
      , validator: v => (['right', 'up', 'left', 'down'].indexOf(v) > -1)
      , default: 'left'
    }
  }
  , data(){ return ({
    openState: this.startOpen
  })}
  , components
  , computed
  , watch
  , methods
}
</script>

<style lang="sass" scoped>
.drawer
  position: absolute
  z-index: 1
  // min-width: 60px
  padding: 1.5em
  // padding-left: 2em
  border: 1px solid darken($grey, 10)
  background: rgba(0, 0, 0, 0.3)
  backdrop-filter: blur(3px)
  box-shadow: 1px 2px 3px rgba(0, 0, 0, 0.3)

  transition: transform 0.4s ease

  .collapse-bar
    position: absolute
    border: none
    overflow: hidden
    background: transparentize($background, 0.1)
    padding: 0
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

  &.direction-left
    right: 0
    border-radius: 4px 0 0 4px
    border-right: none
    &.collapsed
      transform: translate3d(calc(100% - 20px), 0, 0)

    .collapse-bar
      top: 0
      left: 0
      bottom: 0
      width: 20px
      border-radius: 4px 0 0 4px

  &.direction-right
    left: 0
    border-radius: 0 4px 4px 0
    border-left: none
    &.collapsed
      transform: translate3d(calc(-100% + 20px), 0, 0)

    .collapse-bar
      top: 0
      right: 0
      bottom: 0
      width: 20px
      border-radius: 0 4px 4px 0
</style>
