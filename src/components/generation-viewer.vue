<template lang="pug">
.generation-viewer
  v3-renderer(
    ref="renderer"
    , :width="viewWidth"
    , :height="viewHeight"
    , :shadows="true"
  )
    v3-scene
      //- v3-camera(
      //-   ref="camera"
      //-   , type="orthographic"
      //-   , :left="-viewWidth/2"
      //-   , :right="viewWidth/2"
      //-   , :top="viewHeight/2"
      //-   , :bottom="-viewHeight/2"
      //-   , :zoom="2"
      //-   , :near="0.01"
      //-   , :far="5000"
      //-   , :position="orthCameraPos"
      //-   , :look-at="origin"
      //- )
      v3-camera(
        ref="camera"
        , :position="persCameraPos"
        , :far="5000"
        , :aspect="viewWidth / viewHeight"
      )
      v3-light(type="ambient", :intensity="0.9")
      v3-light(
        type="directional"
        , :intensity="0.3"
        , :position="[100, 200, -10]"
        , :cast-shadow="true"
        , :shadow-camera="shadowCamera"
        , :shadow-bias="0.0001"
        , :shadow-radius="0"
        , :shadow-map-size-power="4"
      )
      v3-light(
        type="directional"
        , :intensity="0.05"
        , :position="[10, 200, 100]"
      )
      v3-fog(:near="1000", :far="3000", :color="0xFFFFFF")
      //- v3-grid(
      //-   :size="gridSize - 10"
      //-   , :position="[0, 0.01, 0]",
      //-   , :divisions="50"
      //-   , :color1="0x999999"
      //-   , :color2="0x999999"
      //- )
      //- Board
      v3-plane(:width="gridSize", :height="gridSize", :position="[0, 0, 0]", :color="0xBBBBBB", :rotation="[-Math.PI / 2, 0, 0]", :receive-shadow="true")
      //- Thick board undernieth
      v3-box(:width="gridSize + 40", :height="gridSize + 40", :depth="10", :position="[0, -5.2, 0]", :color="0xAAAAAA", :rotation="[-Math.PI / 2, 0, 0]", :receive-shadow="true")
      v3-group(:position="[-gridSize * 0.5, 0, -gridSize * 0.5]")
        Food(v-for="(food, index) in generation.food", :key="index", :food="food", :cast-shadow="true", :receive-shadow="true")
      v3-group(:position="[-gridSize * 0.5, 0, -gridSize * 0.5]")
        Creature(v-for="(c, index) in generation.creatures", :key="index", :creature="c", :size="3")
</template>

<script>
import { mapGetters } from 'vuex'
import * as THREE from 'three'
import v3Renderer from '@/components/three-vue/v3-renderer'
import v3Scene from '@/components/three-vue/v3-scene'
import v3Camera from '@/components/three-vue/v3-camera'
import v3Light from '@/components/three-vue/v3-light'
import v3Group from '@/components/three-vue/v3-group'
import v3Dom from '@/components/three-vue/v3-dom'
import v3Grid from '@/components/three-vue/v3-grid'
import v3Plane from '@/components/three-vue/v3-plane'
import v3Box from '@/components/three-vue/v3-box'
import v3Fog from '@/components/three-vue/v3-fog'
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
  , v3Plane
  , v3Box
  , v3Fog

  , Food
  , Creature
}

const computed = {
  generation(){
    if ( !this.results ){ return {} }
    return this.results.generations[this.generationIndex]
  }
  , steps(){
    return this.generation.steps
  }
  , ...mapGetters('simulation', {
    'results': 'results'
  })
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
    controls.maxDistance = 2500
    let epsilon = 0.001
    controls.minPolarAngle = epsilon
    controls.maxPolarAngle = Math.PI - epsilon
  }
  , draw(){
    this.time = this.getTime()
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
    generationIndex: {
      type: Number
      , default: 0
    }
    , stepTime: Number
  }
  , inject: [ 'getTime' ]
  , data: () => ({
    viewWidth: 500
    , viewHeight: 500
    , gridSize: 500
    , origin: [0, 0, 0]
    , persCameraPos: [600, 300, 600]
    , orthCameraPos: [100, 50, 100]
    , time: 0
    , shadowCamera: {
      near: 10
      , far: 500
      , left: -300
      , right: 300
      , top: 300
      , bottom: -300
    }
  })
  , components
  , computed
  , watch
  , methods
  , mounted(){
    this.$onResize(() => this.onResize())
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
  background: #333333
</style>
