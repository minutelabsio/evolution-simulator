<template lang="pug">
.floating-panel(:class="{ 'is-open': isOpen, ['direction-' + direction]: true }")
  .activator(@click="isOpen = true")
    slot(name="activator")
      b-icon.icon-btn(:icon="openIcon", :size="size")
  transition(name="fade")
    .panel(v-if="isOpen", @click="closeOnClick && (isOpen = false)")
      .close(@click.stop="isOpen = false")
        slot(name="deactivator")
          b-icon.icon-btn(:icon="closeIcon", :size="size")
      slot
</template>

<script>
const components = {

}

const computed = {
  closeIcon(){
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
  , isOpen: {
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
  name: 'FloatingPanel'
  , props: {
    open: {
      type: Boolean
      , default: undefined
    }
    , closeOnClick: {
      type: Boolean
      , default: true
    }
    , size: String
    , direction: {
      type: String
      , validator: v => (['right', 'up', 'left', 'down'].indexOf(v) > -1)
      , default: 'down'
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
.floating-panel
  position: relative
  display: inline-block
  &.is-open .activator
    visibility: hidden
.panel
  position: absolute
  z-index: 1
  padding: 0.5rem
  border-radius: 4px
  border: 1px solid darken($grey, 10)
  background: rgba(0, 0, 0, 0.3)
  backdrop-filter: blur(3px)
  box-shadow: 2px 2px 3px rgba(0, 0, 0, 0.3)
  display: flex
  flex-direction: column
  align-items: center
  .close
    display: flex

.direction-down
  .panel
    top: -0.5rem
    left: 50%
    transform: translateX(-50%)

    .close
      border-bottom: 1px solid $grey-dark
      padding-bottom: 0.5rem

    >>> .item
      margin-top: 0.5rem
.direction-up
  .panel
    flex-direction: column-reverse
    left: 50%
    bottom: -0.5rem
    transform: translateX(-50%)

    .close
      border-top: 1px solid $grey-dark
      padding-top: 0.5rem

    >>> .item
      margin-bottom: 0.5rem
.direction-left
  .panel
    flex-direction: row-reverse
    bottom: -0.4em
    right: -0.5rem

    .close
      border-left: 1px solid $grey-dark
      padding-left: 0.5rem

    >>> .item
      margin-right: 0.5rem

.direction-right
  .panel
    flex-direction: row
    bottom: -0.4em
    left: -0.5rem

    .close
      border-right: 1px solid $grey-dark
      padding-right: 0.5rem

    >>> .item
      margin-left: 0.5rem
</style>
