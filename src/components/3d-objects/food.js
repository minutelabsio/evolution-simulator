import * as THREE from 'three'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'
import sougy from '@/config/sougy-colors'
import chroma from 'chroma-js'

const foodSize = 2
const foodColor = chroma(sougy.green).darken(1).saturate(0.2).num()

const foodGeometry = new THREE.SphereGeometry( foodSize, 64, 64 )

export default {
  name: 'food'
  , mixins: [ THREEObjectMixin ]
  , inject: [ 'getStep' ]
  , props: {
    food: Object
  }
  , data: () => ({
  })
  , watch: {
    food(){
      this.foodObject.material.opacity = 0
    }
  }
  , created(){
    this.beforeDraw(() => {
      let step = this.getStep()
      let isEaten = step >= this.food.status.Eaten
      this.foodObject.visible = !isEaten

      let material = this.foodObject.material
      material.opacity = THREE.Math.lerp(material.opacity, 1, 0.1)
      // this.v3object.material.opacity = isEaten ? 0.2 : 1
    })
  }
  , methods: {
    createObject(){
      this.v3object = new THREE.Group()
      const foodMaterial = new THREE.MeshLambertMaterial({
        transparent: true
        , opacity: 0
        , color: foodColor
      })
      this.registerDisposables(foodMaterial)
      this.foodObject = new THREE.Mesh( foodGeometry, foodMaterial )
      this.v3object.add(this.foodObject)
    }
    , updateObjects(){
      let pos = this.food.position
      this.v3object.position.set(pos[0], foodSize + 0.2, pos[1])
    }
  }
}
