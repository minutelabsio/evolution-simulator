const app = import('@/wasm/pkg/app')

// Helpers
// ---------------------------------------
function log( ...args ){
  if ( process.env.NODE_ENV !== 'production' ){
    console.log.apply(console, args)
  }
}

function logErr( ...args ){
  if ( process.env.NODE_ENV !== 'production' ){
    console.error.apply(console, args)
  }
}

async function run( method, ...args ){
  const wasm = await app

  const fn = wasm[method]
  if ( !fn ){
    throw new Error(`Method ${method} not defined`)
  }

  try {
    log(`Worker: Running ${method}`, args)
    return fn.apply( wasm, args )
  } catch( msg ){
    // wasm bingen doesn't throw true js errors....
    let err = new Error( msg )
    logErr(`Worker: ERROR from ${method}`, err)
    throw err
  }
}

// API
// ---------------------------------------
export async function test(){
  return run('test')
}
