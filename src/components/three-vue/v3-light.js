import {
  AmbientLight
  , DirectionalLight
  , HemisphereLight
  , PointLight
  , RectAreaLight
  , SpotLight
  // , CameraHelper
  // , Vector3
} from 'three'
import THREEObjectMixin from './v3-object.mixin'

const Types = {
  ambientlight: ({ color, intensity }) => new AmbientLight( color, intensity )
  , directionallight: ({ color, intensity }) => new DirectionalLight( color, intensity )
  , hemispherelight: ({ skyColor, groundColor, intensity }) => new HemisphereLight( skyColor, groundColor, intensity )
  , pointlight: ({ color, intensity, distance, decay }) => new PointLight( color, intensity, distance, decay )
  , rectarealight: ({ color, intensity, width, height }) => new RectAreaLight( color, intensity, width, height )
  , spotlight: ({ color, intensity, distance, angle, penumbra, decay }) => new SpotLight( color, intensity, distance, angle, penumbra, decay )
}

const watchableProps = {
  color: {
    type: Number
    , default: 0xffffff
  }
  , intensity: {
    type: Number
    , default: 1
  }
  , distance: {
    type: Number
    , default: 0
  }
  , angle: {
    type: Number
    , default: Math.PI/2
  }
  , penumbra: {
    type: Number
    , default: 0
  }
  , decay: {
    type: Number
    , default: undefined
  }
  , skyColor: {
    type: Number
    , default: 0xffffff
  }
  , groundColor: {
    type: Number
    , default: 0xffffff
  }
  , position: {
    default: () => [ 0, 0, 0 ]
  }
}

export default {
  name: 'v3-light'
  , mixins: [ THREEObjectMixin ]
  , props: {
    type: String
    , target: String

    , ...watchableProps
  }
  , components: {
  }
  , data: () => ({
  })
  , created(){
    const light = this.lightConstructor( this )
    // if (this.type === 'directional'){
    //   light.castShadow = true
    //   light.shadow.mapSize.width = 512
    //   light.shadow.mapSize.height = 512
    //
    //   light.shadow.camera.near = 10
    //   light.shadow.camera.far = 500
    //   let d = 100
    //   light.shadow.camera.left = -d
    //   light.shadow.camera.right = d
    //   light.shadow.camera.top = d
    //   light.shadow.camera.bottom = -d
    //   light.shadow.bias = 1
    //
    //   let helper = new CameraHelper( light.shadow.camera )
    //   this.$parent.v3object.add( helper )
    //   this.$on('hook:beforeDestroy', () => {
    //     this.$parent.v3object.remove( helper )
    //   })
    // }

    this.v3object = light
  }
  , watch: {
    target( val ){
      if ( val ){
        this.threeVue.afterReady(() => {
          let obj = this.threeVue.getObjectByName( val )
          this.v3object.target = obj
        })
      }
    }
  }
  , computed: {
    lightConstructor(){
      let name = this.type.toLowerCase()
      return Types[ name + 'light' ] || Types[ name ]
    }
  }
  , methods: {
    updateObjects(){
      this.assignProps( this.v3object, watchableProps )
    }
  }
}
