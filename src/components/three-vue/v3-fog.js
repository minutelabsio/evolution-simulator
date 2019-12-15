import * as THREE from 'three'

const threeProps = {
  color: {
    type: Number
    , default: 0x000000
  }
  , near: {
    type: Number
    , default: 1
  }
  , far: {
    type: Number
    , default: 1000
  }
}

export default {
  name: 'v3-fog'
  , inject: [ 'threeVue' ]
  , props: {
    ...threeProps
  }
  , beforeDestroy(){
    this.threeVue.scene.fog = null
  }
  , render(h){
    this.threeVue.scene.fog = new THREE.Fog( this.color, this.near, this.far )
    return h('div')
  }
}
