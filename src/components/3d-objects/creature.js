import * as THREE from 'three'
import Copilot from 'copilot'
import _pick from 'lodash/pick'
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
    , default: 0xff6666
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

export default {
  name: 'creature'
  , mixins: [ THREEObjectMixin ]
  , props: {
    ...threeProps
    , ...materialProps
    , size: {
      type: Number
      , default: 3
    }
    , creature: Object
    , time: Number
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
    geometry(){
      return new THREE.SphereGeometry( this.size, 32, 32 )
    }
    , creaturePosition(){
      let pos = this.v3object.position
      let stepFrac = this.time / this.stepTime
      let p = creaturePositionAt(this.creature, stepFrac)
      pos.set(p[0], 0, p[1])
      return pos
    }
  }
  , methods: {
    createObject(){
      let options = _pick(this, Object.keys(materialProps))
      let material = new THREE.MeshBasicMaterial({ side: THREE.DoubleSide, ...options })
      this.v3object = new THREE.Mesh( this.geometry, material )
    }
    , updateObjects(){
      this.assignProps( this.v3object, threeProps )
      this.assignProps( this.v3object.material, materialProps )
      this.v3object.position.copy(this.creaturePosition)
    }
  }
}
