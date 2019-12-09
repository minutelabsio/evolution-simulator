import * as THREE from 'three'
import Copilot from 'copilot'
import _pick from 'lodash/pick'
import { MarchingCubes } from 'three/examples/jsm/objects/MarchingCubes'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'

const threeProps = {
  position: {
    default: () => [0, 0, 0]
  }
  , rotation: {
    default: () => [0, 0, 0]
  }
}

const materialProps = {
  color: {
    type: Number
    , default: 0x476B81
  }
  , transparent: {
    type: Boolean
    , default: false
  }
  , opacity: {
    type: Number
    , default: 1
  }
}

function lerpArray(from, to, t){
  return Copilot.Interpolators.Array(from, to, t)
}

function creaturePositionAt(creature, stepFrac) {
  let step = Math.floor(stepFrac)
  let frac = stepFrac % Math.max(1, step)
  let from = creature.movement_history[step] || creature.movement_history[creature.movement_history.length - 1]
  let to = creature.movement_history[step + 1]

  if ( !to ){ return from }
  return lerpArray(from, to, frac)
}

function makeEye(size){
  let geo = new THREE.SphereGeometry( size, 16, 16, Math.PI / 2, Math.PI )
  let material = new THREE.MeshBasicMaterial({ color: 0x000000 })
  return new THREE.Mesh( geo, material )
}

export default {
  name: 'creature'
  , mixins: [ THREEObjectMixin ]
  , inject: [ 'getTime' ]
  , props: {
    ...threeProps
    , ...materialProps
    , size: {
      type: Number
      , default: 3
    }
    , creature: Object
    , stepTime: Number
  }
  , components: {
  }
  , data: () => ({
  })
  , watch: {
    geometry( geometry, oldGeometry ){
      // cleanup
      oldGeometry.dispose()
    }
  }
  , computed: {
    steps(){
      return this.creature.movement_history.length
    }
    , spline(){
      return new THREE.SplineCurve(this.creature.movement_history.map(p => {
        return new THREE.Vector2(p[0], p[1])
      }))
    }
    , geometry(){
      return new THREE.SphereGeometry( this.size, 32, 32 )
    }
  }
  , created(){
    this.beforeDraw(() => {
      let time = this.getTime()
      let pos = this.v3object.position
      let stepFrac = time / this.stepTime
      // let p = creaturePositionAt(this.creature, stepFrac)
      // pos.set(p[0], 0, p[1])
      let t = Math.min(stepFrac / this.steps, 1)
      this.spline.getPoint(t, this.tmpV2)
      pos.set(this.tmpV2.x, 0, this.tmpV2.y)

      let rot = this.v3object.rotation
      let ang = this.spline.getTangent(t).angle()
      rot.set(0, -ang, 0)
    })
  }
  , methods: {
    createObject(){
      this.tmpV2 = new THREE.Vector2()

      const size = 40
      const resolution = 60
      const isolation = 300
      let options = _pick(this, Object.keys(materialProps))
      const material = new THREE.MeshLambertMaterial({ ...options })
      const effect = this.effect = new MarchingCubes(resolution, material, true, true)
      effect.scale.set(size, size, size)
      effect.isolation = isolation

      let strength = 1.2 / ( ( Math.sqrt( 3 ) - 1 ) / 4 + 1 )
      effect.reset()
      effect.addBall(0.5, 0.5, 0.5, strength, 100)
      effect.addBall(0.52, 0.54, 0.5, strength/8, 10)
      effect.addBall(0.515, 0.58, 0.5, strength/4, 10)

      this.v3object = new THREE.Group()
      // this.v3object.add(effect)

      let geo = effect.generateBufferGeometry()
      let cached = new THREE.Mesh( geo, material )
      cached.scale.set(size, size, size)
      this.v3object.add(cached)

      // eyes
      let x = 0.082
      let right = makeEye(size / 85)
      right.position.set(size * x, size / 6, size / 30)
      right.rotation.set(-0.6, -0.6, 0)
      this.v3object.add(right)

      let left = makeEye(size / 85)
      left.position.set(size * x, size / 6, -size / 30)
      left.rotation.set(0.6, 0.6, 0)
      this.v3object.add(left)
    }
    , updateObjects(){
      this.assignProps( this.v3object, threeProps )
      this.assignProps( this.effect.material, materialProps )
    }
  }
}
