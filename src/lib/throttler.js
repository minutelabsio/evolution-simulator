import Promise from 'bluebird'

export default class Throttler {
  constructor( delay, batchSize = 1 ){
    this.delay = delay
    this.batchSize = batchSize
    this.count = 0
  }

  schedule( arg ){
    this.count++
    let batch = Math.floor(this.count / this.batchSize)
    let delay = batch * this.delay
    return Promise.delay( delay ).tap( () => (this.count--) ).return( arg )
  }
}
