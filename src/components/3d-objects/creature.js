import * as THREE from 'three'
import _times from 'lodash/times'
import chroma from 'chroma-js'
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
  blob.position.y = 4

  // eyes
  let x = 0.082
  let right = makeEye(size / 85)
  right.name = 'right-eye'
  right.position.set(size * x, size / 4.2, size / 30)
  right.rotation.set(-0.6, -0.6, 0)

  let left = makeEye(size / 85)
  left.name = 'left-eye'
  left.position.set(size * x, size / 4.2, -size / 30)
  left.rotation.set(0.6, 0.6, 0)

  let rightDead = makeDeadEye(size / 40)
  rightDead.name = 'right-dead-eye'
  rightDead.position.set(size * x, size / 4.2, size / 25)
  rightDead.rotation.set(-0.8, -0.7, -0.35)

  let leftDead = makeDeadEye(size / 40)
  leftDead.name = 'left-dead-eye'
  leftDead.position.set(size * x, size / 4.2, -size / 25)
  leftDead.rotation.set(0.8, 0.7, -0.35)

  return [blob, left, right, rightDead, leftDead]
}

const cachedBlobParts = createBlobCreatureParts()

const createBlob = () => cachedBlobParts.reduce(
  (group, part) => group.add(part.clone(false))
  , new THREE.Group()
)

function createCircle(r, color = 'white'){
  let geometry = new THREE.CircleGeometry( r, 64 )
  let material = new THREE.MeshBasicMaterial({ color })
  let circle = new THREE.Mesh( geometry, material )
  return circle
}

const cachedVisionCircle = (() => {
  let c = createCircle(1, chroma.mix('#3a85a8', 'white', 0.8).num())
  c.rotation.x = -Math.PI / 2
  // c.position.y = 0.05
  c.material.depthWrite = false
  c.material.transparent = true
  c.material.blending = THREE.MultiplyBlending
  return c
})()

const cachedEnergyCircle = (() => {
  let c = createCircle(6)
  c.rotation.x = -Math.PI / 2
  // c.position.y = 0.06
  c.material.depthWrite = false
  c.material.transparent = true
  // c.material.blending = THREE.MultiplyBlending
  return c
})()

function getChevronShape(t = 0.4, ang = 45){
  ang *= Math.PI / 180
  let sin = Math.sin(ang)
  let cos = Math.cos(ang)
  let s = new THREE.Shape()
  s.lineTo(-1 * cos, -1 * sin)
  s.lineTo(-1 * cos + t * sin, -1 * sin - t * cos)
  s.lineTo(0, - t / cos)
  s.lineTo(1 * cos - t * sin, -1 * sin - t * cos)
  s.lineTo(1 * cos, -1 * sin)
  s.lineTo(0, 0)
  return s
}

const cachedSpeedIndicator = (() => {
  let g = new THREE.ShapeGeometry( getChevronShape(0.35, 30) )
  let m = new THREE.MeshBasicMaterial({ color: chroma.mix('#c4635d', 'white', 0.15).num() })
  // m.depthWrite = false
  // m.transparent = true
  // m.blending = THREE.MultiplyBlending
  m.depthFunc = THREE.AlwaysDepth
  let obj = new THREE.Group()
  _times(4, n => {
    let growth = 1
    let spaceBetween = 2
    let chev = new THREE.Mesh(g, m)
    let c = n + 1
    let s = Math.sqrt(c) * c * growth + 1
    chev.rotation.set(-Math.PI / 2, 0, -Math.PI / 2)
    chev.position.set(s * spaceBetween - 4, 0, 0)
    chev.scale.set(s, s, s)
    obj.add(chev)
  })

  return obj
})()

const getSpeedIndicator = () => cachedSpeedIndicator.children.reduce(
  (group, part) => group.add(part.clone(false))
  , new THREE.Group()
)

const energyColorScale = chroma.scale(['rgba(255, 200, 29, 0.8)', 'rgba(76, 69, 49, 0.1)']).mode('lab')
function getEnergyColor(creature, progress){
  let start = 0
  let end = creature.energy_consumed / creature.energy
  let s = THREE.Math.lerp(start, end, progress)
  return energyColorScale(s)
}

function getSpeedIndicatorCount(creature){
  let speed = creature.speed[0]
  let maxSpeed = 50
  let s = speed / maxSpeed
  return THREE.Math.lerp(1, 4, s) | 0
}

export default {
  name: 'creature'
  , mixins: [ THREEObjectMixin ]
  , inject: [ 'getStep' ]
  , props: {
    showSenseRange: Boolean
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
      pos.set(this.tmpV2.x, 0, this.tmpV2.y)

      let rot = this.v3object.rotation
      let ang = this.spline.getTangent(t).angle()
      rot.set(0, -ang, 0)

      let deadState = (t >= 1 && this.willDie)
      this.creatureObject.rotation.x = deadState ? Math.PI / 2 : 0
      this.creatureObject.position.y = deadState ? 2 : 0
      this.blobMaterial.opacity = deadState ? 0.3 : 1
      this.blob.castShadow = !deadState

      this.rightEye.visible = !deadState
      this.leftEye.visible = !deadState
      this.rightDeadEye.visible = deadState
      this.leftDeadEye.visible = deadState

      // energy color
      let energyColor = getEnergyColor(this.creature, t)
      this.energyIndicator.material.color.set(energyColor.num())
      this.energyIndicator.material.opacity = energyColor.alpha()
    })
  }
  , methods: {
    createObject(){
      this.tmpV2 = new THREE.Vector2()
      this.v3object = new THREE.Group()
      this.creatureObject = createBlob()
      this.v3object.add(this.creatureObject)
      let blob = this.v3object.getObjectByName('blob')
      this.blob = blob
      blob.material = blob.material.clone()
      this.blobMaterial = blob.material
      this.blobMaterial.transparent = true
      this.registerDisposables(blob.material)

      this.leftEye = this.creatureObject.getObjectByName('left-eye')
      this.rightEye = this.creatureObject.getObjectByName('right-eye')
      this.leftDeadEye = this.creatureObject.getObjectByName('left-dead-eye')
      this.rightDeadEye = this.creatureObject.getObjectByName('right-dead-eye')

      // vision radius
      // material and geometry are reused between creatures
      let visionIndicator = this.visionIndicator = cachedVisionCircle.clone(false) // createCircle( 1, 0xe2ebef )
      this.v3object.add(visionIndicator)

      // energy indicator (geometry reused)
      let energyIndicator = this.energyIndicator = cachedEnergyCircle.clone(false)
      energyIndicator.material = cachedEnergyCircle.material.clone()
      this.registerDisposables([ energyIndicator, energyIndicator.material ])
      this.v3object.add(energyIndicator)

      let speedIndicator = this.speedIndicator = getSpeedIndicator() //cachedSpeedIndicator.clone(false)
      speedIndicator.position.set(9, 0.05, 0)
      this.v3object.add(speedIndicator)

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
    , updateObjects(){
      let vision = this.creature.sense_range[0]
      this.visionIndicator.scale.set(vision, vision, vision)
      this.visionIndicator.visible = true //this.showSenseRange

      // speed indication
      let chevN = getSpeedIndicatorCount(this.creature)
      this.speedIndicator.children.forEach((c, n) => {
        c.visible = (n < chevN)
      })
    }
  }
}
