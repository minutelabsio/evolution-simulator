import * as THREE from 'three'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'
import sougy from '@/config/sougy-colors'
import chroma from 'chroma-js'

const foodSize = 2
const foodColor = chroma(sougy.green).darken(0.8).num()

export default {
  name: 'food'
  , mixins: [ THREEObjectMixin ]
  , inject: [ 'getStep' ]
  , props: {
    food: Object
  }
  , data: () => ({
  })
  , created(){
    this.beforeDraw(() => {
      let step = this.getStep()
      let isEaten = step >= this.food.status.Eaten
      this.v3object.visible = !isEaten
      // this.v3object.material.opacity = isEaten ? 0.2 : 1
    })
  }
  , methods: {
    createObject(){
      let material = new THREE.MeshLambertMaterial({
        transparent: true
      })
      let geometry = new THREE.SphereGeometry( foodSize, 64, 64 )
      this.v3object = new THREE.Mesh( geometry, material )
    }
    , updateObjects(){
      this.v3object.material.color = new THREE.Color(foodColor)
      let pos = this.food.position
      this.v3object.position.set(pos[0], foodSize + 0.2, pos[1])
    }
  }
}
