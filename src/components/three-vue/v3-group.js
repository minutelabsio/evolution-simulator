import * as THREE from 'three'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'

const threeProps = {
  position: {
    default: () => [0, 0, 0]
  }
  , rotation: {
    default: () => [0, 0, 0]
  }
}

export default {
  name: 'v3-group'
  , mixins: [ THREEObjectMixin ]
  , props: {
    ...threeProps
  }
  , components: {
  }
  , data: () => ({
  })
  , created(){
    this.v3object = new THREE.Group()
  }
  , methods: {
    updateObjects(){
      this.assignProps( this.v3object, threeProps )
    }
  }
}
