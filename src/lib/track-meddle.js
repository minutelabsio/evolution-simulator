import _debounce from 'lodash/debounce'

export default _debounce(function(sim, prop, val){
  if ( typeof val === 'string' ){
    prop = prop + ':' + val
    val = undefined
  } else {
    val = val | 0
  }
  sim.$ga.event(
    sim.$options.name
    , 'meddle'
    , prop
    , val
  )
}, 1000)
