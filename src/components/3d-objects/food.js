import * as THREE from 'three'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'

export default {
  name: 'food'
  , mixins: [ THREEObjectMixin ]
  , inject: [ 'getStep' ]
  , props: {
    food: Object
  }
  , data: () => ({
    color: 0x6E8044
  })
  , created(){
    this.beforeDraw(() => {
      let step = this.getStep()
      let isEaten = step > this.food.status.Eaten
      this.v3object.visible = !isEaten
      // this.v3object.material.opacity = isEaten ? 0.2 : 1
    })
  }
  , methods: {
    createObject(){
      let material = new THREE.MeshBasicMaterial({
        side: THREE.DoubleSide
        , transparent: true
      })
      let geometry = new THREE.SphereGeometry( 1, 64, 64 )
      this.v3object = new THREE.Mesh( geometry, material )

    }
    , updateObjects(){
      this.v3object.material.color = new THREE.Color(this.color)
      let pos = this.food.position
      this.v3object.position.set(pos[0], 1, pos[1])
    }
  }
}
