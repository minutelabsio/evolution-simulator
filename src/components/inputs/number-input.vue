<template lang="pug">
.number-input.control.input(
  @click="$refs.input.focus()"
  , :class="{ condensed }"
)
  .label
    span {{ label }}
  .controls
    .slider
      vue-slider(
        v-model="sliderValue"
        , :min="-1"
        , :max="1"
        , :interval="0.1"
        , tooltip="none"
        , :process="sliderProcess"
        , :contained="true"
        , @drag-start="sliderDrag(true)"
        , @change="sliderDrag(sliderIsDragging)"
        , @drag-end="sliderDrag(false)"
      )
    .in
      input(ref="input", type="number", v-model.number="displayValue", :step="step")
</template>

<script>
import chroma from 'chroma-js'
import _throttle from 'lodash/throttle'

function nearest(v, s){
  let figs = Math.max(0, (`${s}`).length - 2)
  return +((Math.round(v / s) * s).toFixed(figs))
}

export default {
  name: 'NumberInput'
  , props: {
    label: String
    , value: Number
    , condensed: Boolean
    , color: [String, Number]
    , trueColor: Boolean
    , step: {
      type: Number
      , default: 1
    }
    , changeRate: {
      type: Number
      , default: 10
    }
    , min: {
      type: Number
    }
    , max: {
      type: Number
    }
  }
  , data: () => ({
    sliderValue: 0
    , sliderIsDragging: false
    , sliderProcess: dotsPos => [[50, dotsPos[0]]],
  })
  , components: {

  }
  , created(){
    this.lastUpdate = 0
  }
  , mounted(){
  }
  , computed: {
    backgroundColor(){
      if (!this.color){ return '' }
      return chroma(this.color).alpha(this.trueColor ? 0.8 : 0.15)
    }
    , borderColor(){
      if (!this.color){ return '' }
      return chroma(this.color).alpha(this.trueColor ? 0.8 : 0.3)
    }
    , displayValue: {
      get(){
        return this.sanitizeValue(this.value)
      }
      , set(v){
        this.$emit('input', this.sanitizeValue(v))
      }
    }
  }
  , watch: {
    sliderIsDragging(drag){
      if (drag){
        this.changeValue()
      }
    }
    , borderColor: {
      handler(c){
        if (!c){ return }
        this.$nextTick(() => {
          this.$el.style.setProperty('--border-color', c.css())
          this.$el.style.setProperty('--border-color-hover', c.alpha(this.trueColor ? 1 : 0.4).css())
          this.$el.style.setProperty('--border-color-focus', c.alpha(this.trueColor ? 1 : 0.6).css())
        })
      }
      , immediate: true
    }
  }
  , methods: {
    sanitizeValue(v){
      v = nearest(v, this.step)
      if (this.min !== undefined){
        v = Math.max(this.min, v)
      }
      if (this.max !== undefined){
        v = Math.min(this.max, v)
      }
      return v
    }
    , sliderDrag(drag){
      this.sliderIsDragging = !!drag
      if (!drag && this.sliderValue){
        this.$nextTick(() => {
          this.sliderValue = 0
        })
      }
    }
    , changeValue: _throttle(function(){
      if (!this.sliderIsDragging){ return }
      let now = window.performance.now()
      let delay = 50
      let dt = 0

      let v = this.sliderValue

      if (v === 0){ return this.changeValue() }

      let dv = v * v * this.changeRate / 10

      let frac = this.step / dv

      if (frac > 1){
        dt = Math.min(frac * delay, 1000)
      }

      if ((+this.lastUpdate + dt) < now){
        this.displayValue += Math.max(this.step, dv) * (v >= 0 ? 1 : -1)
        this.lastUpdate = now
      }

      this.changeValue()
    }, 50)
  }
}
</script>

<style lang="sass" scoped>
.number-input
  --default-border-color: #{$grey-darker}
  --default-border-color-hover: #{desaturate(darken($cream, 60), 30)}
  --default-border-color-focus: #{desaturate(darken($cream, 55), 30)}
  --border-color: var(--default-border-color)
  --border-color-hover: var(--default-border-color-hover)
  --border-color-focus: var(--default-border-color-focus)

  >>> .vue-slider-process
    background-color: $ivory
  >>> .vue-slider-rail
    background-color: var(--border-color-hover)

  display: flex
  align-items: stretch
  padding: 0
  transition: border 0.15s ease
  font-family: $family-monospace
  border-color: var(--border-color)
  height: auto
  width: 23rem
  background: $black-bis

  @media screen and (max-width: $tablet)
    width: 100%

  &:hover,
  &:active
    border-color: var(--border-color-hover)
    box-shadow: none
  &:focus,
  &:focus-within
    border-color: var(--border-color-focus)

  .controls
    display: flex

  .slider
    display: flex
    align-items: center
    width: 100px
    margin-left: calc(0.625em - 1px)

    &:before
      content: '-'
      position: relative
      top: -1px
    &:after
      content: '+'
    >>> .vue-slider
      flex: 1

  .label,
  input
    padding-bottom: calc(0.375em - 1px)
    padding-left: calc(0.625em - 1px)
    padding-right: calc(0.625em - 1px)
    padding-top: calc(0.375em - 1px)

  .label
    margin-bottom: 0
    font-weight: normal
    color: $text
    background: $black-ter //desaturate(darken($cream, 76), 33) // desaturate(darken($blue, 21), 40)
    border-radius: 3px 0 0 3px
    box-shadow: 1px 0 0px $black-bis
    display: flex
    align-items: center
    justify-content: center
    min-width: 8rem
    text-align: center
    text-shadow: 0 0 1px var(--border-color-focus)

  .in
    flex: 1

  input
    font-size: 1rem
    color: $text
    background: none
    height: 100%
    width: 100%
    border: none
    // flex: 1
    border-radius: 3px
    text-align: center
    font-family: $family-monospace
    padding: 2px 0 0

    &::-webkit-inner-spin-button,
    &::-webkit-outer-spin-button
      -webkit-appearance: none
      margin: 0

    &:focus
      border: none
      outline: none
      background: none

  &.condensed
    width: auto
    .label
      min-width: 5rem
    .controls
      width: 8rem
      padding: 4px 0
      flex-direction: column-reverse

    .slider
      width: auto
      margin-right: calc(0.625em - 1px)

</style>

<style lang="sass">
.number-input-group
  display: flex
  flex-direction: column

  > .number-input
    position: relative
    z-index: 0
    border-radius: 0
    border-color: var(--default-border-color)
    overflow: hidden

    &:after
      content: ''
      pointer-events: none
      position: absolute
      top: 0
      left: 0
      bottom: 0
      right: 0
      z-index: 1
      transition: box-shadow .15s ease

    &:hover,
    &:active
      border-color: var(--default-border-color)

      &:after
        box-shadow: inset 0 0 2px var(--border-color-hover)

    &:focus,
    &:focus-within
      border-color: var(--default-border-color)
      &:after
        box-shadow: inset 0 0 2px var(--border-color-focus)
    &:first-child
      border-radius: 3px 3px 0 0
    &:last-child
      border-radius: 0 0 3px 3px
    &:not(:last-child)
      border-bottom: none
    // &:hover + .number-input,
    // &:active + .number-input
    //   border-top-color: var(--default-border-color-hover)
    // &:focus + .number-input,
    // &:focus-within + .number-input
    //   border-top-color: var(--default-border-color-focus)
</style>
