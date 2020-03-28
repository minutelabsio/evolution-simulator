import {
  AmbientLight
  , DirectionalLight
  , HemisphereLight
  , PointLight
  , RectAreaLight
  , SpotLight
  // , CameraHelper
  , Vector2
} from 'three'
import _mapKeys from 'lodash/mapKeys'
import THREEObjectMixin from './v3-object.mixin'

const Types = {
  ambientlight: ({ color, intensity }) => new AmbientLight( color, intensity )
  , directionallight: ({ color, intensity }) => new DirectionalLight( color, intensity )
  , hemispherelight: ({ skyColor, groundColor, intensity }) => new HemisphereLight( skyColor, groundColor, intensity )
  , pointlight: ({ color, intensity, distance, decay }) => new PointLight( color, intensity, distance, decay )
  , rectarealight: ({ color, intensity, width, height }) => new RectAreaLight( color, intensity, width, height )
  , spotlight: ({ color, intensity, distance, angle, penumbra, decay }) => new SpotLight( color, intensity, distance, angle, penumbra, decay )
}

const lightProps = {
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

const shadowProps = {
  shadowBias: {
    type: Number
    , default: 0
  }
  , shadowRadius: {
    type: Number
    , default: 1
  }
}

const shadowPropsNoPrefix = _mapKeys(shadowProps, (v, k) => k[6].toLowerCase() + k.substr(7))

export default {
  name: 'v3-light'
  , mixins: [ THREEObjectMixin ]
  , props: {
    type: String
    , target: String

    , ...lightProps
    , ...shadowProps
    , shadowMapSizePower: {
      type: Number
      , default: 0
    }
    , shadowCamera: {
      type: Object
      , default: () => ({})
    }
  }
  , components: {
  }
  , data: () => ({
  })
  , created(){
    const light = this.lightConstructor( this )

    // let shadowHelper = new CameraHelper( light.shadow.camera )
    // this.$parent.v3object.add( shadowHelper )
    // this.beforeDraw(() => {
    //   shadowHelper.update()
    // })
    // this.$on('hook:beforeDestroy', () => {
    //   this.$parent.v3object.remove( shadowHelper )
    // })

    this.v3object = light

    this.registerDisposables({
      dispose: () => {
        if (!this.v3object.shadow){ return }
        if (this.v3object.shadow.map){
          this.v3object.shadow.map.dispose()
        }
        if (this.v3object.shadow.mapPass){
          this.v3object.shadow.mapPass.dispose()
        }
      }
    })
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
      this.assignProps( this.v3object, lightProps )
      if ( this.castShadow ){
        this.assignProps( this.v3object.shadow, shadowPropsNoPrefix )
        this.v3object.shadow.mapSize = new Vector2(512, 512)
        this.v3object.shadow.mapSize.multiplyScalar(Math.pow(2, this.shadowMapSizePower))
        if ( this.v3object.shadow.map ){
          this.v3object.shadow.map.dispose()
          this.v3object.shadow.map = null
        }
        Object.assign(this.v3object.shadow.camera, this.shadowCamera)
      }
    }
  }
}
