import * as THREE from 'three'
import _get from 'lodash/get'

export default {
  name: 'Gestures'
  , inject: [ 'threeVue' ]
  , props: {
    names: Array
  }
  , components: {
  }
  , data: () => ({
  })
  , created(){
    this.raycaster = new THREE.Raycaster()
    this.pos = new THREE.Vector2()
  }
  , mounted(){

    const updateMonitored = () => {
      if (!this.names){
        this.monitored = []
        return
      }

      let names = this.names

      function collectNamedChildren( result, obj ){
        if ( !obj ){ return result }

        if ( names.indexOf(obj.name) > -1 ){
          result.push( obj )
        }

        return obj.children.reduce( collectNamedChildren, result )
      }

      this.monitored = this.threeVue.scene.children.reduce( collectNamedChildren, [] )
    }

    this.threeVue.$on('scene:changed', updateMonitored)
    this.$on('beforeDestroy', () => {
      this.threeVue.$off('scene:changed', updateMonitored)
    })

    updateMonitored()

    this.listen('mousemove', e => this.onMouseMove( e ))
    this.listen('mousedown', e => this.onMouseDown( e ))
    this.listen('mouseup', e => this.onMouseUp( e ))

    this.listen('touchmove', e => this.onTouchMove( e ))
    this.listen('touchstart', e => this.onTouchStart( e ), false)
    this.listen('touchend', e => this.onTouchEnd( e ))
  }
  , render(){
    return null
  }
  , methods: {
    listen( name, fn, flag ){
      let el = this.threeVue.renderer.domElement

      if ( name === 'mouseup' || name === 'touchend' ){
        el = document
      }

      el.addEventListener(name, fn, flag !== undefined ? flag : { passive: true })
      this.$on('hook:beforeDestroy', () => {
        el.removeEventListener(name, fn)
      })
    }
    , getMousePos( e ){
      let pos = this.pos
      let elOffset = this.threeVue.renderer.domElement.getBoundingClientRect()
      let x = e.clientX - elOffset.left
      let y = e.clientY - elOffset.top

      pos.x = ( x / elOffset.width ) * 2 - 1
      pos.y = - ( y / elOffset.height ) * 2 + 1

      return pos
    }
    , getTouchPos( e ){
      let pos = this.pos
      let elOffset = this.threeVue.renderer.domElement.getBoundingClientRect()
      let touch = e.touches[0]

      if ( !touch ){
        return pos
      }

      let x = touch.pageX - elOffset.left
      let y = touch.pageY - elOffset.top

      pos.x = ( x / elOffset.width ) * 2 - 1
      pos.y = - ( y / elOffset.height ) * 2 + 1

      return pos
    }
    , raycast( pos ){
      const camera = this.threeVue.camera
      const raycaster = this.raycaster

      if ( !camera ){ return [] }

      // update the picking ray with the camera and mouse position
      raycaster.setFromCamera( pos, camera )

      // calculate objects intersecting the picking ray
      return raycaster.intersectObjects( this.monitored )
    }
    , onMouseDown( e ){
      let pos = this.getMousePos( e )
      this.dragStart( pos )
    }
    , onMouseUp( e ){
      let pos = this.getMousePos( e )
      this.dragEnd( pos )
    }
    , onMouseMove( e ){
      let pos = this.getMousePos( e )
      this.drag( pos )
    }
    , onTouchStart( e ){
      e.preventDefault()
      let pos = this.getTouchPos( e )
      this.dragStart( pos )
    }
    , onTouchEnd( e ){
      let pos = this.getTouchPos( e )
      this.dragEnd( pos )
    }
    , onTouchMove( e ){
      e.stopPropagation()
      let pos = this.getTouchPos( e )
      this.drag( pos )
    }
    , dragStart( pos ){
      let intersects = this.raycast( pos )

      if ( !intersects.length ){ return }

      let ray = this.raycaster.ray.clone()

      this.dragging = { intersects, ray }
      this.dragged = false

      this.$emit('dragstart', { intersects, ray })
    }
    , dragEnd( pos ){
      let intersects = this.raycast( pos )

      let dragging = this.dragging
      this.dragging = false

      if ( !dragging ){ return }

      let ray = this.raycaster.ray.clone()

      if ( this.dragged ){
        this.$emit('dragend', { intersects, ray })
      } else if (_get(intersects[0], 'object') === _get(dragging.intersects[0], 'object')){
        this.$emit('tap', { intersects, ray })
      }
    }
    , drag( pos ){
      let intersects = this.raycast( pos )

      let ray = this.raycaster.ray.clone()

      if ( this.dragging ){
        this.dragged = true
        this.$emit('drag', { intersects, ray })
      } else {
        this.$emit('hover', { intersects, ray })
      }
    }
  }
}
