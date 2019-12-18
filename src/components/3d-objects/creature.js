import * as THREE from 'three'
import { MarchingCubes } from 'three/examples/jsm/objects/MarchingCubes'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'

const blobColor = 0x476B81

function makeEye(size){
  let geo = new THREE.SphereGeometry( size, 16, 16, Math.PI / 2, Math.PI )
  let material = new THREE.MeshBasicMaterial({ color: 0x000000 })
  return new THREE.Mesh( geo, material )
}

function makeDeadEye(size){
  let material = new THREE.MeshBasicMaterial({ color: 0x888888 })
  let thickness = size / 8
  let h = new THREE.Mesh(new THREE.PlaneGeometry(size, thickness), material)
  h.rotation.set(0, Math.PI/2, 0)
  let v = new THREE.Mesh(new THREE.PlaneGeometry(thickness, size), material)
  v.rotation.set(0, Math.PI/2, 0)
  let g = new THREE.Group()
  g.add(h, v)
  return g
}

function createBlobCreatureParts(){
  const size = 40
  const resolution = 160
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
  effect.material.dispose()
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

  let rightDead = makeDeadEye(size / 40)
  rightDead.name = 'right-dead-eye'
  rightDead.position.set(size * x, size / 6, size / 23)
  rightDead.rotation.set(-0.8, -0.7, -0.35)

  let leftDead = makeDeadEye(size / 40)
  leftDead.name = 'left-dead-eye'
  leftDead.position.set(size * x, size / 6, -size / 23)
  leftDead.rotation.set(0.8, 0.7, -0.35)

  return [blob, left, right, rightDead, leftDead]
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
    , willDie(){
      return this.creature.state === "DEAD"
    }
  }
  , created(){
    this.beforeDraw(() => {
      let pos = this.v3object.position
      let stepFrac = this.getStep()
      let t = Math.min(stepFrac / this.steps, 1)
      this.spline.getPoint(t, this.tmpV2)
      pos.set(this.tmpV2.x, 2.8, this.tmpV2.y)

      let rot = this.v3object.rotation
      let ang = this.spline.getTangent(t).angle()
      rot.set(0, -ang, 0)

      let deadState = (t >= 1 && this.willDie)
      rot.x = deadState ? Math.PI / 2 : 0
      this.blobMaterial.opacity = deadState ? 0.3 : 1
      this.blob.castShadow = !deadState

      this.rightEye.visible = !deadState
      this.leftEye.visible = !deadState
      this.rightDeadEye.visible = deadState
      this.leftDeadEye.visible = deadState
    })
  }
  , methods: {
    createObject(){
      this.tmpV2 = new THREE.Vector2()
      this.v3object = createBlob()
      let blob = this.v3object.getObjectByName('blob')
      this.blob = blob
      blob.material = blob.material.clone()
      this.blobMaterial = blob.material
      this.blobMaterial.transparent = true
      this.registerDisposables(blob.material)

      this.leftEye = this.v3object.getObjectByName('left-eye')
      this.rightEye = this.v3object.getObjectByName('right-eye')
      this.leftDeadEye = this.v3object.getObjectByName('left-dead-eye')
      this.rightDeadEye = this.v3object.getObjectByName('right-dead-eye')

      // const scene = this.threeVue.scene
      // let light = this.light = new THREE.SpotLight( 0xFFFFFF, 1, 0, Math.PI / 24 )
      // light.penumbra = 0.2
      // light.target = blob
      // scene.add(light)
      //
      // this.beforeDraw(() => {
      //   blob.getWorldPosition(light.position)
      //   light.position.y += 200
      // })
      //
      // this.$on('hook:beforeDestroy', () => {
      //   scene.remove(light)
      // })

      // Path
      // let points = new THREE.BufferGeometry()
      // let line = new THREE.Points(points)
      // this.$parent.v3object.add(line)
      // this.$watch('spline', () => {
      //   let vertices = this.spline.points.reduce((r, p) => r.push(p.x, 0, p.y) && r, [])
      //   points.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))
      //   points.attributes.position.needsUpdate = true
      // }, { immediate: true })
      //
      // this.$on('hook:beforeDestroy', () => {
      //   this.$parent.v3object.remove(line)
      // })
    }
  }
}
