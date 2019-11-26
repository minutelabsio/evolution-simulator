const app = import('@/wasm/pkg/app')
app.then( mod => {
  console.log('setting debug hook')
  mod.browser_debug()
})

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
export async function runSimulation( cfg, creatures ){
  return run('run_simulation', cfg, creatures)
}

export async function continueSimulation( cfg, creatures ){
  return run('continue_simulation', cfg, creatures)
}

// export async function initRandomCreatures( cfg, creatureCfg ){
//   return run('init_random_creatures', cfg, creatureCfg)
// }
