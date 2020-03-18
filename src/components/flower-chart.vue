<template lang="pug">
.flower-chart(
  :style="{ width: size + 'px', height: size + 'px', transform: 'rotate(-0.25turn)' }"
  , :class="{ 'show-center-value': centerHover, 'show-values': showValues, 'no-hover': valuesOutside }"
)
  svg(ref="svg", :viewBox="viewbox", :width="size", :height="size")
    circle.outer(:r="valuesOutside ? 0.5 : 1", fill="none")
    g.petals(:transform="`rotate(${globalAngle})`")
      g.petal(v-for="(svg, index) in petalSVG", v-bind="svg.group", @click="onPetalClick(index)")
        path.hover-area(v-bind="svg.hoverArea")
        path(v-bind="svg.petal")
        text(v-bind="svg.text") {{ svg.value | fixed(2) }}
    g.center(v-if="showCenter", @mouseenter="centerHover = true && !showValues", @mouseleave="centerHover = false", @click="onCenterClick")
      circle(:r="center", :fill="colors.center")
      circle.hover-area(r="0.3")
      text(transform="rotate(90)", :dy="showValues ? 0.03 : -0.3", alignment-baseline="middle") {{ data.center }}
</template>

<script>
import chroma from 'chroma-js'

const Pi2 = Math.PI * 2

function scale(min, max, z){
  if ( max === min ){ return 1 }
  return (z - min) / (max - min)
}

function lerp(a, b, t) {
  return a * (1 - t) + b * t
}

function petalPath(r, arc){
  let x = Math.cos(0.5 * arc)
  let y = Math.sin(0.5 * arc)
  // M x0 y0
  // A rx ry x-axis-rotation large-arc-flag sweep-flag x y
  // L x2 y2
  return `M ${r * x} ${-r * y} A ${r} ${r} 0 0 1 ${r * x} ${r * y} L 0 0`
}

export default {
  name: 'FlowerChart'
  , filters: {
    fixed(v, n){
      if (v === undefined){ return '' }
      return v.toFixed(n)
    }
  }
  , props: {
    size: {
      type: Number
      , default: 50
    }
    , data: {
      type: Object
      , default: () => ({})
    }
    , colors: {
      type: Object
      , default: () => ({
        center: '#e6e6e6'
        , petals: chroma.scale('Set1').colors(8)
      })
    }
    , dataRanges: {
      type: Object
      , default: () => ({
        center: [0, 1]
        , petals: [0, 1]
      })
    }
    , petalWidth: {
      type: Number
      , default: 1 - 1 / 1.618
    }
    , showValues: {
      type: Boolean
      , default: false
    }
    , topPetal: {
      type: Number
      , default: 0
    }
    , showCenter: {
      type: Boolean
      , default: true
    }
    , valuesOutside: {
      type: Boolean
      , default: false
    }
  }
  , data: () => ({
    viewbox: '-1.1 -1.1 2.2 2.2'
    , centerHover: false
  })
  , mounted(){
  }
  , watch: {
  }
  , computed: {
    globalAngle(){
      return -360 * this.topPetal / this.petals.length
    }
    , center(){
      return this.centerScale(this.data.center) || 0
    }
    , centerScale(){
      let size = 0.25
      let range = this.dataRanges.center
      let min = range[0]
      let max = range[1]
      return v => size * scale(min, max, v)
    }
    , petals(){
      return (this.data.petals || []).map((value, i) => ({
        value
        , scaled: this.petalScales[i](value) * (this.valuesOutside ? 0.5 : 1)
      }))
    }
    , petalScales(){
      let minSize = this.centerScale(this.dataRanges.center[1])
      let ranges = this.dataRanges.petals

      return this.data.petals.map((p, i) => {
        let range = ranges[i]
        if ( !Array.isArray(range) ){
          range = ranges
        }
        let min = range[0]
        let max = range[1]
        return v => lerp(minSize, 1, scale(min, max, v))
      })
    }
    , petalSVG(){
      let colors = this.colors.petals
      let colorSpaceSize = colors.length
      let len = this.petals.length
      let frac = len / this.petalWidth
      let ang = Pi2 / frac
      let ga = this.globalAngle

      return this.petals.map((p, i) => {
        // M x0 y0
        // A rx ry x-axis-rotation large-arc-flag sweep-flag x y
        // L x2 y2
        let fill = colors[i % colorSpaceSize]
        let d = petalPath(p.scaled, ang)
        let rot = 360 * (i / len)
        let transform = `rotate(${rot})`

        let group = { transform }
        let petal = { d, fill }

        d = petalPath(1, Pi2 / len)

        let hoverArea = { d, fill }

        let textFill = chroma.mix(fill, this.valuesOutside ? 'black' : 'white', 0.6).css()
        let text = {
          transform: `translate(0.8, 0) rotate(${-rot - ga + 90})`
          , 'alignment-baseline': 'middle'
          , style: { stroke: textFill, fill: textFill }
        }

        return {
          group
          , petal
          , hoverArea
          , text
          , value: p.value
        }
      })
    }
  }
  , methods: {
    onPetalClick(index){
      let data = this.data.petals[index]
      this.$emit('select', {
        petal: true
        , index
        , data
      })
    }
    , onCenterClick(){
      let data = this.data.center
      this.$emit('select', {
        center: true
        , data
      })
    }
  }
}
</script>

<style lang="sass" scoped>
.flower-chart
  display: inline-block
svg
  font-size: .3px
text
  fill: white
  stroke: black
  stroke-width: 0
  text-anchor: middle
  text-shadow: 0 0 1px white
.petals
  transition: transform 0.5s ease
.petal
  transition: all 0.15s ease
  cursor: pointer
  path
    transition: fill 0.15s ease
  .hover-area
    opacity: 0.05
    transition: opacity 0.15s ease
  text
    opacity: 0
    transition: opacity 0.15s ease, transform 0.5s ease
  &:hover
    .hover-area
      opacity: 0.2
    text
      opacity: 1
circle
  stroke-width: 0.03
  stroke: #222
circle.outer
  stroke-width: 0.01
  stroke: rgba(160, 160, 160, 0.1)
.center
  cursor: pointer
  text
    opacity: 0
    transition: opacity 0.15s ease
  .hover-area
    fill: white
    stroke-width: 0
    opacity: 0
.show-center-value
  .petal
    opacity: 0.1
    path,
    text
      fill: grey
  .center
    text
      opacity: 1
.no-hover
  .center, .petal
    text
      opacity: 1
// overrides to show values only
.show-values
  .petal,
  .center
    .hover-area
      opacity: 1
    text
      opacity: 1
  .center
    text
      fill: $grey-dark
</style>
