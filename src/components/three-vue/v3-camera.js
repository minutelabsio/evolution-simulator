import {
  OrthographicCamera
  , PerspectiveCamera
  , Vector3
} from 'three'
import THREEObjectMixin from './v3-object.mixin'

const Types = {
  orthographiccamera: ({ left, right, top, bottom, near, far }) => new OrthographicCamera( left, right, top, bottom, near, far )
  , perspectivecamera: ({ fov, aspect, near, far }) => new PerspectiveCamera( fov, aspect, near, far )
}

const watchableProps = {
  fov: {
    type: Number
    , default: 45
  }
  , aspect: {
    type: Number
    , default: 1
  }
  , near: {
    type: Number
    , default: 1
  }
  , far: {
    type: Number
    , default: 1000
  }
  , left: Number
  , right: Number
  , top: Number
  , bottom: Number

  , zoom: {
    type: Number
    , default: 1
  }

  , position: {
    default: () => [ 0, 0, 0 ]
  }
  , rotation: {
    default: () => [ 0, 0, 0 ]
  }
}

export default {
  name: 'v3-camera'
  , mixins: [ THREEObjectMixin ]
  , props: {
    type: {
      type: String
      , default: 'perspective'
    }

    , lookAt: Array
    , ...watchableProps
  }
  , components: {
  }
  , data: () => ({
  })
  , created(){
    const camera = this.camera = this.cameraConstructor( this )
    camera.position.z = 1

    this.v3object = camera

    this.$watch(() => {
      // watch these
      return '' + this.fov +
        this.aspect +
        this.left +
        this.right +
        this.top +
        this.bottom +
        this.zoom
    }, () => {
      this.$nextTick(() => this.v3object.updateProjectionMatrix())
    })
  }
  , mounted(){
    this.v3object.updateProjectionMatrix()
    this.threeVue.camera = this.v3object
  }
  , computed: {
    lookAtV(){
      return new Vector3().fromArray(this.lookAt)
    }
    , cameraConstructor(){
      let name = this.type.toLowerCase()
      return Types[ name + 'camera' ] || Types[ name ]
    }
  }
  , methods: {
    updateObjects(){
      this.assignProps( this.v3object, watchableProps )
      if ( this.lookAt ){
        this.v3object.lookAt(this.lookAtV)
      }
    }
  }
}
