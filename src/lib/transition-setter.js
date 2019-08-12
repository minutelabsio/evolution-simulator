import Copilot from 'copilot'

export default function TransitionSetter( opts ){

  const hasSetter = opts.hasSetter
  const onUpdate = opts.onUpdate
  const getCurrent = opts.getCurrent
  const duration = opts.duration || 1000
  let currentProgress = 1
  let current = opts.current || 0
  let prev = hasSetter ? current.clone() : 0

  const start = function( val ){
    currentProgress = 0
    if ( hasSetter ){
      prev.copy( val )
    } else {
      prev = val
    }
  }

  return {
    start
    , update( delta ){
      let to = getCurrent( current )

      if ( currentProgress >= 1 ){
        currentProgress = 1
      } else {
        currentProgress += delta / duration
      }

      let progress = Copilot.Easing.Quadratic.InOut( currentProgress )
      onUpdate( prev, to, progress )
    }
  }
}
