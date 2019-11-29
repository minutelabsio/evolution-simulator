<template lang="pug">
.generation-viewer
  v3-renderer(
    ref="renderer"
    , :width="viewWidth"
    , :height="viewHeight"
  )
    v3-scene
      v3-camera(
        ref="camera"
        , type="orthographic"
        , :left="-viewWidth/2"
        , :right="viewWidth/2"
        , :top="viewHeight/2"
        , :bottom="-viewHeight/2"
        , :zoom="2"
        , :near="0.01"
        , :far="5000"
        , :position="orthCameraPos"
        , :look-at="origin"
      )
      v3-light(type="ambient", :intensity="0.7")
      v3-grid(
        :size="gridSize"
        , :divisions="50"
        , :color1="0x001e44"
        , :color2="0x102b61"
      )

      Food(v-for="food in foods", v-bind="food")
      v3-group(:position="[-gridSize * 0.5, 3, -gridSize * 0.5]")
        Creature(v-for="c in generation.creatures", :creature="c", :size="3", :time="time", :step-time="stepTime")
</template>

<script>
import * as THREE from 'three'
import v3Renderer from '@/components/three-vue/v3-renderer'
import v3Scene from '@/components/three-vue/v3-scene'
import v3Camera from '@/components/three-vue/v3-camera'
import v3Light from '@/components/three-vue/v3-light'
import v3Group from '@/components/three-vue/v3-group'
import v3Dom from '@/components/three-vue/v3-dom'
import v3Grid from '@/components/three-vue/v3-grid'
import Food from '@/components/3d-objects/food'
import Creature from '@/components/3d-objects/creature'
const OrbitControls = require('three-orbit-controls')(THREE)

const components = {
  v3Renderer
  , v3Scene
  , v3Camera
  , v3Light
  , v3Group
  , v3Dom
  , v3Grid

  , Food
  , Creature
}

const computed = {
  foods(){
    if (!this.generation.food){ return [] }
    return this.generation.food.map(f => {
      let radius = 1
      let position = this.toViewCoords(f.position[0], f.position[1])
      position[1] += radius
      return {
        position
        , radius
      }
    })
  }
}

const watch = {
}

const methods = {
  debug(){
    const scene = this.$refs.renderer.scene
    // The X axis is red. The Y axis is green. The Z axis is blue.
    var axesHelper = new THREE.AxesHelper( 5 )
    scene.add( axesHelper )
  }
  , initCamera(){
    const renderer = this.$refs.renderer.renderer
    const camera = this.$refs.camera.v3object
    // controls
    let controls = this.controls = new OrbitControls( camera, renderer.domElement )
    controls.rotateSpeed = 0.2
    controls.zoomSpeed = 1.2
    controls.panSpeed = 0.8
    controls.enableZoom = true
    controls.enablePan = false
    // controls.staticMoving = true
    controls.enableDamping = true
    controls.dampingFactor = 0.1
    controls.minZoom = 1
    controls.maxZoom = 500
    let epsilon = 0.001
    controls.minPolarAngle = epsilon
    controls.maxPolarAngle = Math.PI - epsilon
  }
  , draw(){
    this.controls.update()
    this.$refs.renderer.draw()
  }
  , onResize(){
    let el = this.$el
    this.viewWidth = el.offsetWidth
    this.viewHeight = el.offsetHeight
  }
  , toViewCoords(x = 0, y = 0){
    let hw = 0.5 * this.gridSize
    return [x - hw, 0, y - hw]
  }
}

export default {
  name: 'GenerationViewer'
  , props: {
    generation: {
      type: Object
      , default: {}
    }
    , time: Number
    , stepTime: Number
  }
  , data: () => ({
    viewWidth: 500
    , viewHeight: 500
    , gridSize: 500
    , origin: [0, 0, 0]
    , orthCameraPos: [1000, 500, 1000]
  })
  , components
  , computed
  , watch
  , methods
  , mounted(){
    // TODO resizing
    this.onResize()
    this.initCamera()
    this.debug()

    // Initialize drawing
    let stop = false
    const clock = new THREE.Clock()
    const draw = () => {
      if ( stop ) { return }
      requestAnimationFrame( draw )
      this.draw( clock.getDelta() * 1000 )
    }
    this.$on('hook:beforeDestroy', () => {
      stop = true
    })
    draw()
  }
}
</script>

<style lang="sass" scoped>
.generation-viewer
  width: 100%
  height: 100%
</style>
