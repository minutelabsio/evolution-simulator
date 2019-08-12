import * as THREE from 'three'
/* eslint-disable no-unused-vars */
import { CSS2DRenderer } from 'three/examples/js/renderers/CSS2DRenderer'
import { EffectComposer } from 'three/examples/js/postprocessing/EffectComposer'
import { CopyShader } from 'three/examples/js/shaders/CopyShader'
// import { FXAAShader } from 'three/examples/js/shaders/FXAAShader'
import { RenderPass } from 'three/examples/js/postprocessing/RenderPass'
import { ShaderPass } from 'three/examples/js/postprocessing/ShaderPass'
import { OutlinePass } from 'three/examples/js/postprocessing/OutlinePass'
/* eslint-enable no-unused-vars */
/* eslint-disable comma-style */
// BUG fix for memory leak....
THREE.Pass.FullScreenQuad = ( function () {

	var camera = new THREE.OrthographicCamera( - 1, 1, 1, - 1, 0, 1 );

	var FullScreenQuad = function ( material ) {
    var geometry = new THREE.PlaneBufferGeometry( 2, 2 );

		this._mesh = new THREE.Mesh( geometry, material );

	};

	Object.defineProperty( FullScreenQuad.prototype, 'material', {

		get: function () {

			return this._mesh.material;

		},

		set: function ( value ) {

			this._mesh.material = value;

		}

	} );

	Object.assign( FullScreenQuad.prototype, {

		render: function ( renderer ) {

			renderer.render( this._mesh, camera );

		}

	} );

	return FullScreenQuad;

} )();
/* eslint-enable comma-style */

export default {
  name: 'v3-renderer'
  , props: {
    width: Number
    , height: Number
    , outlineColor: {
      type: Number
      , default: 0xFFFFFF
    }
    , outlineColorBehind: {
      type: Number
      , default: 0x666666
    }
  }
  , components: {
  }
  , data: () => ({
  })
  , provide(){
    const threeVue = this

    return { threeVue }
  }
  , created(){

    this.afterReady(() => { this.isReady = true })

    this.renderer = new THREE.WebGLRenderer({
      alpha: true
      , antialias: true
      // , canvas: this.$el
    })

    this.renderer.setPixelRatio( window.devicePixelRatio )
    // this.renderer.toneMapping = THREE.ReinhardToneMapping

    this.cssRenderer = new CSS2DRenderer({})

    // https://threejs.org/examples/webgl_postprocessing_outline.html
    let composer = this.composer = new THREE.EffectComposer( this.renderer )
    let renderPass = this.renderPass = new THREE.RenderPass( null, null )
    composer.addPass( renderPass )

    // let effectFXAA = new THREE.ShaderPass( THREE.FXAAShader )
    // effectFXAA.uniforms[ 'resolution' ].value.set( 1 / this.width, 1 / this.height )
    // composer.addPass( effectFXAA )

    this.$watch(() => this.width + ~this.height, () => {
      this.renderer.setSize( this.width, this.height )
      this.cssRenderer.setSize( this.width, this.height )
      this.composer.setSize( this.width, this.height )
      // effectFXAA.uniforms[ 'resolution' ].value.set( 1 / this.width, 1 / this.height )
      this.$emit('resize')
    }, { immediate: true })
  }
  , beforeDestroy(){
    this.renderer.dispose()
    this.composer.renderTarget1.dispose()
    this.composer.renderTarget2.dispose()
    this.composer.copyPass.material.dispose()
    this.composer.copyPass.fsQuad._mesh.geometry.dispose()
    if ( this.outlinePass ){
      this.outlinePass.dispose()
    }
  }
  , mounted(){
    // append renderers
    this.cssRenderer.domElement.style.position = 'absolute'
    this.cssRenderer.domElement.style.top = '0'
    this.cssRenderer.domElement.style.left = '0'
    this.cssRenderer.domElement.style.zIndex = '1'
    this.cssRenderer.domElement.style.userSelect = 'none'
    this.cssRenderer.domElement.className = 'no-events'
    this.$el.appendChild( this.renderer.domElement )
    this.$el.appendChild( this.cssRenderer.domElement )
  }
  , computed: {
  }
  , watch: {
    outlineColor(){
      this.outlinePass.visibleEdgeColor.set(this.outlineColor)
    }
    , outlineColorBehind(){
      this.outlinePass.hiddenEdgeColor.set(this.outlineColorBehind)
    }
  }
  , methods: {
    draw(){
      if ( !this.scene ){
        this.draw = () => {}
        throw new Error('No scene added to the renderer')
      }

      if ( !this.camera ){
        this.draw = () => {}
        throw new Error('No camera added to the renderer')
      }

      this.$emit('beforeDraw')

      this.initOutlinePass()

      this.renderPass.camera = this.camera
      this.renderPass.scene = this.scene
      // this.renderer.render( this.scene, this.camera )
      this.composer.render()
      this.cssRenderer.render( this.scene, this.camera )
    }
    , getObjectByName( name ){
      return this.scene.getObjectByName( name )
    }
    , afterReady( fn ){
      if ( this.isReady ){
        fn()
        return this
      }

      this.$once('hook:mounted', fn)
    }
    , checkClear(){
      let idx = this.composer.passes.indexOf(this.renderPass)
      this.renderPass.clear = idx === 0
    }
    , insertRenderPass( pass, idx ){
      if ( this.composer.passes.indexOf( pass ) > -1 ){ return }
      if ( idx === undefined ){
        idx = this.composer.passes.length
      }

      this.composer.insertPass( pass, idx )
      this.checkClear()
    }
    , removeRenderPass( pass ){
      let idx = this.composer.passes.indexOf( pass )
      if ( idx < 0 ){ return }
      this.composer.passes.splice( idx, 1 )
    }
    , initOutlinePass(){
      if (
        !this.outlinePass ||
        this.outlinePass.renderCamera !== this.camera ||
        this.outlinePass.renderScene !== this.scene
      ){
        if ( this.outlinePass ){
          let idx = this.composer.passes.indexOf(this.outlinePass)
          this.composer.passes.splice( idx, 1 )
          this.outlinePass.dispose()
        }
        const dims = new THREE.Vector2( this.width, this.height )
        let outlinePass = this.outlinePass = new THREE.OutlinePass( dims, this.scene, this.camera )

        outlinePass.edgeStrength = 3
        outlinePass.edgeThickness = 1
        outlinePass.pulsePeriod = 2
        outlinePass.visibleEdgeColor.set(this.outlineColor)
        outlinePass.hiddenEdgeColor.set(this.outlineColorBehind)

        this.composer.addPass( outlinePass )
      }
    }
    , addOutline( v3object ){
      let idx = this.outlinePass.selectedObjects.indexOf( v3object )
      if ( idx > -1 ) return
      this.outlinePass.selectedObjects.push( v3object )
    }
    , removeOutline( v3object ){
      if ( v3object === undefined ){
        // remove all
        this.outlinePass.selectedObjects = []
      }
      let idx = this.outlinePass.selectedObjects.indexOf( v3object )
      if ( idx < 0 ) return
      this.outlinePass.selectedObjects.splice( idx, 1 )
    }
  }
  , render(h){
    return h('div'
      , {
        style: {
          position: 'relative'
          , overflow: 'hidden'
        }
      }
      , this.$slots.default
    )
  }
}
