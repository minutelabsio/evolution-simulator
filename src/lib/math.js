export function RunningStatistics( initialData = [] ){
  let m = 0
  let s = 0
  let n = 0
  let _max = Number.NEGATIVE_INFINITY
  let _min = Number.POSITIVE_INFINITY
  let total = 0

  // Push a value to a running average calculation.
  // see [http://www.johndcook.com/blog/standard_deviation]
  // Note: variance can be calculated from the "s" value by multiplying it by `1/(n-1)`
  function push(v){
    n++
    let x = v - m

    // Mk = Mk-1 + (xk – Mk-1)/k
    // Sk = Sk-1 + (xk – Mk-1)*(xk – Mk).
    m += x / n
    s += x * (v - m)

    // max / min
    _max = Math.max(v, _max)
    _min = Math.min(v, _min)
    total += v
  }

  function mean(){
    return m
  }

  function variance(){
    return s/(n-1)
  }

  function deviation(){
    return Math.sqrt(variance())
  }

  function max(){ return _max }
  function min(){ return _min }
  function sum(){ return total }
  function size(){ return n }

  function toObject(){
    return {
      mean: mean()
      , deviation: deviation()
      , sum: sum()
      , size: size()
      , max: max()
      , min: min()
    }
  }

  for ( let i = 0, l = initialData.length; i < l; i++ ){
    push(initialData[i])
  }

  return {
    mean
    , variance
    , deviation
    , sum
    , size
    , max
    , min
    , push
    , toObject
  }
}

export function rndSimpleGaussian(mean = 0, sigma = 1){
  let g = Math.random() + Math.random() + Math.random()
  return 2 * sigma * (g - 1) + mean - 3
}
