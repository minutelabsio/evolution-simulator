const hiddenProperty = 'hidden' in document ? 'hidden' :
  'webkitHidden' in document ? 'webkitHidden' :
  'mozHidden' in document ? 'mozHidden' : null
// const visibilityStateProperty = 'visibilityState' in document ? 'visibilityState' :
//   'webkitVisibilityState' in document ? 'webkitVisibilityState' :
//   'mozVisibilityState' in document ? 'mozVisibilityState' : null
const visibilityChangeEvent = hiddenProperty && hiddenProperty.replace(/hidden/i, 'visibilitychange')
const isSupported = !!hiddenProperty

export function onChange( fn, args ){
  if ( !isSupported ) return
  window.addEventListener( visibilityChangeEvent, fn, args )
}

export function offChange( fn ){
  if ( !isSupported ) return
  window.removeEventListener( visibilityChangeEvent, fn )
}

export function isHidden(){
  if ( !isSupported ) return false
  return document[hiddenProperty]
}
