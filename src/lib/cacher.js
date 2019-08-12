import Promise from 'bluebird'

const defaultLifetime = 1000 * 60 * 15

function getKey( args ){
  return JSON.stringify( args )
}

function getCached( cache, key ){
  return cache[ key ]
}

function cacheResult( cache, key, result, lifetime ){
  cache[ key ] = {
    expires: Date.now() + lifetime
    , result
  }

  return result
}

function isExpired( entry ){
  return entry.expires < Date.now()
}

function clean( cache ){
  let keys = Object.keys( cache )
  for ( let k of keys ){
    if ( isExpired( cache[k] ) ){
      delete cache[k]
    }
  }
}

export default function( fn, lifetime = defaultLifetime ){
  const cache = {}

  // clean every 10s
  window.setInterval( () => clean( cache ), 1000 * 10 )

  return function( ...args ){
    let key = getKey( args )
    let cachedResult = getCached( cache, key )

    if ( cachedResult && !isExpired( cachedResult ) ){
      return Promise.resolve( cachedResult.result )
    }

    return Promise.resolve( fn( ...args ) )
      .then( result => cacheResult( cache, key, result, lifetime ) )
  }
}
