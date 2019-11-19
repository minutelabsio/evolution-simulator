<template lang="pug">
.flower-chart(:style="{ width: size + 'px', height: size + 'px', transform: 'rotate(-0.25turn)' }")
  svg(ref="svg", :viewBox="viewbox", :width="size", :height="size")
    path(v-for="petal in petalPaths", v-bind="petal")
    circle(:r="center", :fill="colors.center")
    circle.outer(r="1", fill="none")
</template>

<script>
import chroma from 'chroma-js'

const Pi2 = Math.PI * 2

function scale(min, max, z){
  return (z - min) / (max - min)
}

function lerp(a, b, t) {
  return a * (1 - t) + b * t
}

export default {
  name: 'FlowerChart'
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
  }
  , data: () => ({
    viewbox: '-1.1 -1.1 2.2 2.2'
  })
  , mounted(){
  }
  , watch: {
  }
  , computed: {
    center(){
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
      return (this.data.petals || []).map((p, i) => this.petalScales[i](p))
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
    , petalPaths(){
      let colors = this.colors.petals
      let colorSpaceSize = colors.length
      let len = this.petals.length
      let frac = len / this.petalWidth
      let ang = Pi2 / frac
      let x = Math.cos(ang)
      let y = Math.sin(ang)

      return this.petals.map((p, i) => {
        // M x0 y0
        // A rx ry x-axis-rotation large-arc-flag sweep-flag x y
        // L x2 y2
        let fill = colors[i % colorSpaceSize]
        let r = p
        let d = `M ${r} 0 A ${r} ${r} 0 0 1 ${r * x} ${r * y} L 0 0`
        let rot = 360 * (i / len - 0.5 / frac)
        let transform = `rotate(${rot})`

        return { d, fill, transform }
      })
    }
  }
  , methods: {
  }
}
</script>

<style lang="sass" scoped>
.flower-chart
  display: inline-block
circle
  stroke-width: 0.03
  stroke: #222

circle.outer
  stroke-width: 0.01
  stroke: rgba(160, 160, 160, 0.2)
</style>
