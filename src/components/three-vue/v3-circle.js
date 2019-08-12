import * as THREE from 'three'
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

const geometryProps = {
  radius: {
    type: Number
    , default: 10
  }
  , segments: {
    type: Number
    , default: 64
  }
}

const materialProps = {
  color: {
    type: Number
    , default: 0xffffff
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

export default {
  name: 'v3-circle'
  , mixins: [ THREEObjectMixin ]
  , props: {
    ...threeProps
    , ...geometryProps
    , ...materialProps
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
      return new THREE.CircleGeometry( this.radius, this.segments )
    }
  }
  , methods: {
    createObject(){
      let options = _pick(this, Object.keys(materialProps))
      let material = new THREE.MeshBasicMaterial({ side: THREE.DoubleSide, ...options })
      let circle = new THREE.Mesh( this.geometry, material )
      this.v3object = circle
    }
    , updateObjects(){
      this.assignProps( this.v3object, threeProps )
      this.assignProps( this.v3object.material, materialProps )
    }
  }
}
