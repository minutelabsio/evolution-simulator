import Copilot from 'copilot'
import * as THREE from 'three'
import THREEObjectMixin from '@/components/three-vue/v3-object.mixin'

export default {
  name: 'fade-transtion'
  , mixins: [ THREEObjectMixin ]
  , created(){
    this.v3object = new THREE.Object3D()
  }
  , render: function (h) {
    let data = {
      props: {
        css: false
        // , mode: 'out-in'
      }
      , on: {
        enter: this.enter
        , beforeEnter: this.beforeEnter
        , beforeLeave: this.beforeLeave
        , leave: this.leave
        , afterLeave: this.afterLeave
      }
    }
    return h('transition', data, this.$slots.default)
  }
  , methods: {
    beforeEnter() {
      this.$children.forEach( ch => {
        let material = ch.v3object.material
        material.transparent = true
        material.oldOpacity = material.opacity
        material.opacity = 0
      })
    }
    , enter( el, done ){
      this.fade( true, done )
    }
    , beforeLeave(){
      this.$children.forEach( ch => {
        let material = ch.v3object.material
        material.oldOpacity = material.opacity
      })
    }
    , leave( el, done ){
      this.fade( false, done )
    }
    , afterLeave(){
      this.$children.forEach( ch => {
        let material = ch.v3object.material
        material.opacity = material.oldOpacity
        ch.$parent.v3object.remove( ch.v3object )
      })
    }
    , fade( fadeIn, done ){
      const threeVue = this.threeVue
      const duration = 1000
      const startTime = Copilot.Util.now()
      const endTime = startTime + duration
      const fn = Copilot.Interpolators.Linear
      const children = this.$children.concat()
      children.forEach( ch => {
        // override
        ch.$parent.v3object.add( ch.v3object )
      })

      const update = () => {
        let now = Copilot.Util.now()
        let timeFraction = Copilot.Animation.getTimeFraction(startTime, endTime, now)
        let t = Copilot.Easing.Quadratic.InOut(timeFraction)
        children.forEach( ch => {
          let material = ch.v3object.material
          let from = fadeIn ? 0 : material.oldOpacity
          let to = fadeIn ? material.oldOpacity : 0
          let opacity = Copilot.Animation.interpolateProperty(fn, from, to, t)
          material.opacity = opacity
        })

        if ( now > endTime ){
          threeVue.$off('beforeDraw', update)
          done()
        }
      }

      threeVue.$on('beforeDraw', update)
    }
  }
}
