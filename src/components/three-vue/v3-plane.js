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
  width: {
    type: Number
    , default: 10
  }
  , height: {
    type: Number
    , default: 10
  }
  , segments: {
    type: Number
    , default: 1
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
  name: 'v3-plane'
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
      return new THREE.PlaneGeometry( this.width, this.height, this.segments )
    }
  }
  , methods: {
    createObject(){
      let options = _pick(this, Object.keys(materialProps))
      let material = new THREE.MeshBasicMaterial({ side: THREE.DoubleSide, ...options })
      let plane = new THREE.Mesh( this.geometry, material )
      this.v3object = plane
    }
    , updateObjects(){
      this.assignProps( this.v3object, threeProps )
      this.assignProps( this.v3object.material, materialProps )
    }
  }
}
