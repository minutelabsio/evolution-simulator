import {
  Color
  , Vector3
  , Euler
} from 'three'
// import _uniq from 'lodash/uniq'
import _compact from 'lodash/compact'

export default {
  props: {
    name: String
    , visible: {
      type: Boolean
      , default: true
    }
  }
  , inject: [ 'threeVue' ]
  , created(){
    this.disposables = []
  }
  , mounted(){
    if ( !this.v3object ){
      throw new Error('Please set component v3object property')
    }

    const parent = this.$parent.v3object

    if ( !parent ){ return }

    parent.add( this.v3object )
    this.$on('hook:beforeDestroy', () => {
      parent.remove( this.v3object )
    })

    this.threeVue.$emit('scene:changed', { type: 'add', component: this, object: this.v3object })
  }
  , beforeDestroy(){
    this.threeVue.$emit('scene:changed', { type: 'remove', component: this, object: this.v3object })

    this.registerDisposables([ this.v3object, this.v3object.geometry, this.v3object.material ])
    this.disposables.forEach( thing => {
      thing.dispose()
    })
  }
  , render(h){
    if ( !this.v3object ){
      this.createObject()
    }
    this.updateObjects()
    this.v3object.name = this.name
    this.v3object.visible = this.visible
    return h(
      'div'
      , this.visible ? this.$slots.default : []
    )
  }
  , methods: {

    updateObjects(){
      // abstract
    }

    , registerDisposables( thing ){
      if ( !thing || this.disposables.indexOf(thing) > -1 ){ return this }
      if ( thing.dispose ){
        this.disposables.push( thing )

        thing = _compact(thing)
        Array.prototype.push.apply( this.disposables, thing )

      } else if ( Array.isArray( thing ) ){

        for ( let th of thing ){
          this.registerDisposables( th )
        }
      }

      // this.disposables = _uniq(this.disposables)
      return this
    }

    // add frame listner
    , beforeDraw( fn ){
      this.threeVue.$on( 'beforeDraw', fn )

      this.$on('hook:beforeDestroy', () => {
        this.threeVue.$off( 'beforeDraw', fn )
      })
    }

    , assignProps( dest, props ){
      for ( let prop of Object.keys(props) ){
        if ( prop in dest ){
          let val = this[prop]
          let cur = dest[prop]

          if (
            cur instanceof Color ||
            cur instanceof Vector3 ||
            cur instanceof Euler
          ){
            if ( Array.isArray(val) ){
              cur.fromArray( val )
              this.$emit(`update:${prop}`, cur)
            } else if ( typeof val === 'object' ){
              cur.copy( val )
              this.$emit(`update:${prop}`, cur)
            } else {
              cur.set( val )
              this.$emit(`update:${prop}`, cur)
            }
          } else {
            dest[ prop ] = val
            this.$emit(`update:${prop}`, val)
          }
        }
      }
    }
  }
}
