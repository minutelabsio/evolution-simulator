<template lang="pug">
.number-input.control.has-icons-left.input(@click="$refs.input.focus()")
  span.label.is-left Energy
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
  input(ref="input", type="number", v-model.number="displayValue", :step="step")
</template>

<script>
import _throttle from 'lodash/throttle'

function nearest(v, s){
  return Math.round(v / s) * s
}

export default {
  name: 'NumberInput'
  , props: {
    value: Number
    , step: {
      type: Number
      , default: 1
    }
    , changeRate: {
      type: Number
      , default: 10
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
    displayValue: {
      get(){
        return nearest(this.value, this.step)
      }
      , set(v){
        this.$emit('input', nearest(v, this.step))
      }
    }
  }
  , watch: {
    sliderIsDragging(drag){
      if (drag){
        this.changeValue()
      }
    }
  }
  , methods: {
    sliderDrag(drag){
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
      let dv = v * v * this.changeRate

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
>>> .vue-slider-process
  background-color: $yellow
>>> .vue-slider-rail
  background-color: $grey
.number-input
  display: flex
  padding: 0
  transition: border 0.15s ease
  font-family: $family-monospace
  border-color: $grey-dark

  &:hover,
  &:active
    border-color: desaturate(darken($cream, 60), 30)
    box-shadow: none
  &:focus,
  &:focus-within
    border-color: desaturate(darken($cream, 55), 30)

  .slider
    display: flex
    align-items: center
    width: 120px
    margin-left: calc(0.625em - 1px)

    &:before
      content: '-'
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
    height: 100%
    margin-bottom: 0
    font-weight: normal
    color: $text
    background: desaturate(darken($cream, 76), 33) // desaturate(darken($blue, 21), 40)
    border-radius: 3px 0 0 3px
    box-shadow: 1px 0 0px $black-bis

  input
    font-size: 1rem
    color: $text
    background: none
    height: 100%
    border: none
    flex: 1
    border-radius: 3px

    &::-webkit-inner-spin-button,
    &::-webkit-outer-spin-button
      -webkit-appearance: none
      margin: 0

    &:focus
      border: none
      outline: none
      background: none
</style>
