import * as THREE from 'three'
import { CSS2DObject } from 'three/examples/js/renderers/CSS2DRenderer' // eslint-disable-line no-unused-vars
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'

const dummyEl = document.createElement('div')
const origin2d = new THREE.Vector2() // eslint-disable-line no-unused-vars

const threeProps = {
  position: {
    default: () => [0, 0, 0]
  }
  , rotation: {
    default: () => [0, 0, 0]
  }
  , anchorDir: {
    default: () => [0, 0, 0]
  }
}

export default {
  name: 'v3-dom'
  , mixins: [ THREEObjectMixin ]
  , props: {
    ...threeProps
  }
  , components: {
  }
  , data: () => ({
  })
  , created(){
    this.boundingBox = new THREE.Box2()
    // eslint-disable-next-line
    this.v3object = new THREE.CSS2DObject( dummyEl )

    // const vec = this.vec = new THREE.Vector3()
    // const vec2 = this.vec2 = new THREE.Vector2()
    // const vec2b = new THREE.Vector2()
    // const viewProjectionMatrix = new THREE.Matrix4()
    // const viewMatrix = new THREE.Matrix4()
    // this.beforeDraw(() => {
    //   const camera = this.threeVue.camera
    //
    //   viewMatrix.copy( camera.matrixWorldInverse );
		//   viewProjectionMatrix.multiplyMatrices( camera.projectionMatrix, viewMatrix );
    //   vec.fromArray( this.anchorDir ).setLength(100)
		// 	vec.applyMatrix4( viewProjectionMatrix )
    //   this.boundingBox.clampPoint(vec2b.set(vec.x, vec.y), vec2)
    //
    //   this.$el.style.marginTop = `${vec2.y}px`
    //   this.$el.style.marginLeft = `${vec2.x}px`
    // })
  }
  , mounted(){
    this.$el.style.position = 'absolute'
    this.v3object.element = this.$el
    // this.threeVue.$el.appendChild( this.$el )
  }
  , methods: {
    updateObjects(){
      this.assignProps( this.v3object, threeProps )
      // this.getSize()
    }
    // , getSize(){
    //   if ( !this.$el ) return
    //   let box = this.$el.getBoundingClientRect()
    //   this.vec2.set(box.width, box.height)
    //   this.boundingBox.setFromCenterAndSize(
    //     origin2d
    //     , this.vec2
    //   )
    // }
  }
}
