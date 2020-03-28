<template lang="pug">
.generation-viewer
  v3-renderer(
    ref="renderer"
    , :width="viewWidth"
    , :height="viewHeight"
    , :shadows="true"
    , :outlineColor="highlightColor"
  )
    Gestures(
      :names="interactiveObjects"
      , @tap="tapCreature"
      , @hover="onHover"
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

      //- v3-grid(
      //-   :size="gridSize - 10"
      //-   , :position="[0, 0.01, 0]",
      //-   , :divisions="50"
      //-   , :color1="0x999999"
      //-   , :color2="0x999999"
      //- )
      v3-camera(
        ref="camera"
        , :position="persCameraPos"
        , prevent-update
        , :far="5000"
        , :aspect="viewWidth / viewHeight"
      )
        v3-dom(:position="[0, 0, -100]")
          Tour
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
      v3-fog(:near="1000", :far="3000", :color="0xc9d7e6")

      //- Board
      fadeTransition
        v3-plane(v-if="showWorld", :width="gridSize", :height="gridSize", :position="[0, -0.05, 0]", :color="0xFFFFFF", :rotation="[-Math.PI / 2, 0, 0]", :receive-shadow="true")
      fadeTransition
        //- Thick board undernieth
        v3-box(v-if="showWorld", :width="gridSize + 40", :height="gridSize + 40", :depth="10", :position="[0, -6, 0]", :color="0xAAAAAA", :rotation="[-Math.PI / 2, 0, 0]", :receive-shadow="true")

      v3-group(v-if="showWorld", :position="[-gridSize * 0.5, 0, -gridSize * 0.5]")
        Food(v-for="(food, index) in generation.food", :key="index", :food="food", :cast-shadow="true", :receive-shadow="true")
      v3-group(v-if="showWorld", :position="[-gridSize * 0.5, 0, -gridSize * 0.5]")
        Creature(ref="v3Creatures", v-for="(c, index) in generation.creatures", :key="index", :creature="c", :size="3", v-bind="creatureIndicators")
          v3-group(v-if="c.id === followCreatureId")
            v3-dom(:position="[0, 19, 0]")
              CreatureStatus(:creature="c")
            v3-group(:position="[-100, 50, 0]", ref="cameraGoal")
            v3-group(:position="[0, 30, 0]", ref="cameraFocusGoal")
</template>

<script>
import Copilot from 'copilot'
import { mapGetters } from 'vuex'
import chroma from 'chroma-js'
import sougy from '@/config/sougy-colors'
import * as THREE from 'three'
import _throttle from 'lodash/throttle'
import _findIndex from 'lodash/findIndex'
import v3Renderer from '@/components/three-vue/v3-renderer'
import fadeTransition from '@/components/three-vue/fade.transition'
import Gestures from '@/components/three-vue/gestures'
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
import CreatureStatus from '@/components/3d-objects/creature-status'
import Tour from '@/components/tour'
const OrbitControls = require('three-orbit-controls')(THREE)

const components = {
  v3Renderer
  , fadeTransition
  , Gestures
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
  , CreatureStatus
  , Tour
}

const computed = {
  steps(){
    return this.generation.steps
  }
  , generation(){
    return this.getCurrentGeneration()
  }
  , creatureIndicators(){
    return {
      showSightIndicator: this.sightIndicators
      , showSpeedIndicator: this.speedIndicators
      , showEnergyIndicator: this.energyIndicators
      , showFoodIndicator: this.foodIndicators
    }
  }
  , tourStepNumber(){
    return this.$route.query.intro | 0
  }
  , showWorld(){
    return this.generation && !this.hideStage
  }
  , ...mapGetters('simulation', {
    'getCurrentGeneration': 'getCurrentGeneration'
  })
}

const watch = {
  followCreatureId(){
    this.checkFollowCreature()
  }
}

const tmpV = new THREE.Vector3()
const methods = {
  debug(){
    // The X axis is red. The Y axis is green. The Z axis is blue.
    var axesHelper = new THREE.AxesHelper( 5 )
    this.scene.add( axesHelper )
  }
  , initCamera(){
    const renderer = this.$refs.renderer.renderer
    const camera = this.camera = this.$refs.camera.v3object
    // controls
    let controls = this.controls = new OrbitControls( camera, renderer.domElement )
    controls.rotateSpeed = 0.2
    controls.zoomSpeed = 1
    controls.panSpeed = 0.8
    controls.enableZoom = true
    controls.enablePan = false
    // controls.staticMoving = true
    controls.enableDamping = true
    controls.dampingFactor = 0.1
    controls.minZoom = 1
    controls.maxZoom = 500
    controls.maxDistance = 4000
    let epsilon = 0.001
    controls.minPolarAngle = epsilon
    controls.maxPolarAngle = Math.PI - epsilon

    controls.addEventListener('start', () => {
      this.cameraDragging = true
    })

    controls.addEventListener('end', () => {
      this.cameraDragging = false
    })
  }
  , draw(){
    this.followCreature()
    if ( this.transitionCamera && !this.cameraDragging ){
      this.camera.position.lerp(this.cameraGoal, 0.05)
    }
    this.controls.target.copy(this.cameraFocusGoal)

    this.controls.update()
    this.$refs.renderer.draw()
  }
  , followCreature(){
    let goal = this.$refs.cameraGoal && this.$refs.cameraGoal[0]
    let focusGoal = this.$refs.cameraFocusGoal && this.$refs.cameraFocusGoal[0]
    if (!goal){ return }
    // let creature = this.$refs.v3Creatures[this.followCreatureId]
    this.cameraGoal.setFromMatrixPosition(goal.v3object.matrixWorld)
    tmpV.setFromMatrixPosition(focusGoal.v3object.matrixWorld)
    this.cameraFocusGoal.lerp(tmpV, 0.05)
  }
  , checkFollowCreature(){
    let active = this.followCreatureId !== undefined
    // this.controls.enabled = !active
    if (!active){
      this.cameraGoal.fromArray(this.persCameraPos)
      this.cameraFocusGoal.copy(this.scene.position)
      setTimeout(() => {
        this.transitionCamera = false
      }, 1500)
    } else {
      this.transitionCamera = true
    }
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
  , tapCreature({ intersects }){
    if (!intersects.length){ return }
    let blob = intersects[0].object
    let index = _findIndex(this.$refs.v3Creatures, c => c.v3object === blob.parent.parent)
    let creature = this.generation.creatures[index]
    this.$emit('tap-creature', {creature, index})
  }
  , onHover: _throttle(function({ intersects }){
    let renderer = this.renderer
    renderer.removeOutline()
    if (intersects.length){
      let blob = intersects[0].object
      let index = _findIndex(this.$refs.v3Creatures, c => c.v3object === blob.parent.parent)
      let id = this.generation.creatures[index].id

      renderer.addOutline( blob )
      this.$emit('creature-hover', { index, id, creature: blob })
    }
  }, 100)
  , initTour(){
    let frames = Copilot({
      cameraPosition: {
        type: 'Vector3'
        , default: new THREE.Vector3(0, 4000, 300)
        , easing: Copilot.Easing.Quadratic.InOut
      }
      , cameraRotation: {
        type: 'Vector3'
        , default: new THREE.Vector3(0, 0, 0)
      }
      , hideStage: false
    })

    frames.add({}, {
      id: 'step-1'
      , time: 0
    })

    // frames.add({ hideStage: false }, { time: 1, duration: 1 })

    // step 1
    frames.add({
      cameraPosition: new THREE.Vector3().fromArray(this.persCameraPos)
    }, {
      id: 'step-2'
      , time: '5s'
      , duration: '5s'
    })

    this.tourActive = false
    let player = Copilot.Player({ manager: frames })

    frames.on('update', () => {
      if (!this.tourActive){return}
      let state = frames.state

      if (!this.transitionCamera){
        this.camera.position.copy(state.cameraPosition)
      }
      this.hideStage = state.hideStage
    })

    this.$watch('tourStepNumber', (n) => {
      if (!n){
        player.seek(player.totalTime)
        this.tourActive = false
        this.controls.enabled = true
        return
      }

      this.tourActive = true
      this.controls.enabled = false

      let f = frames.getFrame('step-' + n)
      if (!f){ return }

      player.playTo(f.meta.time)
    }, { immediate: true })

    this.$on('hook:beforeDestroy', () => {
      player.destroy()
      frames.off(true)
    })
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
    , sightIndicators: Boolean
    , speedIndicators: Boolean
    , energyIndicators: Boolean
    , foodIndicators: Boolean
    , followCreatureId: String
  }
  , inject: [ 'getTime' ]
  , data: () => ({
    viewWidth: 500
    , viewHeight: 500
    , gridSize: 500
    , origin: [0, 0, 0]
    , persCameraPos: [600, 300, 600]
    , orthCameraPos: [100, 50, 100]
    , shadowCamera: {
      near: 10
      , far: 500
      , left: -300
      , right: 300
      , top: 300
      , bottom: -300
    }
    , cameraGoal: new THREE.Vector3()
    , cameraFocusGoal: new THREE.Vector3()
    , interactiveObjects: ['blob']
    , highlightColor: chroma(sougy.red).num()
    , hideStage: false
  })
  , components
  , computed
  , watch
  , methods
  , mounted(){
    this.renderer = this.$refs.renderer
    this.scene = this.renderer.scene

    this.$onResize(() => this.onResize())
    this.onResize()
    this.initCamera()
    // this.debug()
    this.checkFollowCreature()
    this.initTour()

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
  max-width: 100vw
  background: $grey-darker
</style>
