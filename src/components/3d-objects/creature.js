import * as THREE from 'three'
import { MarchingCubes } from 'three/examples/jsm/objects/MarchingCubes'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'

const blobColor = 0x476B81

function makeEye(size){
  let geo = new THREE.SphereGeometry( size, 16, 16, Math.PI / 2, Math.PI )
  let material = new THREE.MeshBasicMaterial({ color: 0x000000 })
  return new THREE.Mesh( geo, material )
}

function createBlobCreatureParts(){
  const size = 40
  const resolution = 60
  const isolation = 300
  const effect = new MarchingCubes(resolution, new THREE.MeshBasicMaterial(), true, true)
  effect.scale.set(size, size, size)
  effect.isolation = isolation

  let strength = 1.2 / ( ( Math.sqrt( 3 ) - 1 ) / 4 + 1 )
  effect.reset()
  effect.addBall(0.5, 0.5, 0.5, strength, 100)
  effect.addBall(0.52, 0.54, 0.5, strength/8, 10)
  effect.addBall(0.515, 0.58, 0.5, strength/4, 10)

  let geo = effect.generateBufferGeometry()
  let material = new THREE.MeshLambertMaterial({ color: blobColor })
  let blob = new THREE.Mesh( geo, material )
  blob.name = 'blob'
  blob.scale.set(size, size, size)
  blob.position.y = 1

  // eyes
  let x = 0.082
  let right = makeEye(size / 85)
  right.name = 'right-eye'
  right.position.set(size * x, size / 6, size / 30)
  right.rotation.set(-0.6, -0.6, 0)

  let left = makeEye(size / 85)
  left.name = 'left-eye'
  left.position.set(size * x, size / 6, -size / 30)
  left.rotation.set(0.6, 0.6, 0)

  effect.material.dispose()

  return [blob, left, right]
}

const cachedBlobParts = createBlobCreatureParts()

const createBlob = () => cachedBlobParts.reduce(
  (group, part) => group.add(part.clone())
  , new THREE.Group()
)

export default {
  name: 'creature'
  , mixins: [ THREEObjectMixin ]
  , inject: [ 'getStep' ]
  , props: {
    size: {
      type: Number
      , default: 3
    }
    , creature: Object
  }
  , components: {
  }
  , data: () => ({
  })
  , computed: {
    steps(){
      return this.creature.movement_history.length
    }
    , spline(){
      return new THREE.SplineCurve(this.creature.movement_history.map(p => {
        return new THREE.Vector2(p[0], p[1])
      }))
    }
  }
  , created(){
    this.beforeDraw(() => {
      let pos = this.v3object.position
      let stepFrac = this.getStep()
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
      this.v3object = createBlob()
      let blob = this.v3object.getObjectByName('blob')
      blob.material = blob.material.clone()
      this.blobMaterial = blob.material
      this.registerDisposables(blob.material)
    }
  }
}
