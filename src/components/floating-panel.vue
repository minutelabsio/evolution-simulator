<template lang="pug">
.floating-panel(:class="{ 'is-open': isOpen }")
  .activator(@click="isOpen = true")
    slot(name="activator")
      b-icon.icon-btn(icon="menu-down", :size="size")
  transition(name="fade")
    .panel(v-if="isOpen", @click="closeOnClick && (isOpen = false)")
      .close(@click.stop="isOpen = false")
        slot(name="deactivator")
          b-icon.icon-btn(icon="menu-up", :size="size")
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
  .close
    border-bottom: 1px solid $grey-dark
    padding-bottom: 0.5em
.panel
  position: absolute
  top: -0.5em
  left: 50%
  z-index: 1
  padding: 0.5em
  border-radius: 4px
  border: 1px solid darken($grey, 10)
  background: rgba(0, 0, 0, 0.3)
  backdrop-filter: blur(3px)
  box-shadow: 2px 2px 3px rgba(0, 0, 0, 0.3)
  transform: translateX(-50%)
  display: flex
  flex-direction: column
  >>> .item
    margin-top: 0.5em
</style>
