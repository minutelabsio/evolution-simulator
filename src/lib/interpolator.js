function lerp( from, to, t ){
  return from * ( 1 - t ) + to * t
}

// pts is a flat array of the form [x, y, x2, y2, ...]
export default function interpolator(pts, options = {}){
  let { useSpline = false } = options
  // assume to be sorted by x
  const len = pts.length
  const minX = pts[0]
  const maxX = pts[len - 2]

  pts = useSpline ? spline(pts, options.tension, options.isClosed, options.numOfSegments) : pts

  return (x) => {
    x = Math.max(minX, Math.min(maxX, x))
    let i = 0
    let xn = pts[0]
    while (x >= xn){
      i += 2
      xn = pts[i]
    }

    if (i === 0){ return pts[1] }
    if (i === len){ return pts[len - 1]}

    let x1 = pts[i - 2]
    let x2 = pts[i]
    if (x1 === x2){ return pts[i + 1] }
    let t = (x - x1) / (x2 - x1)
    return lerp(pts[i - 1], pts[i + 1], t)
  }
}


// from: https://stackoverflow.com/questions/7054272/how-to-draw-smooth-curve-through-n-points-using-javascript-html5-canvas
function spline(pts, tension = 0.5, isClosed = false, numOfSegments = 16) {

  let _pts = [], res = []    // clone array
  let x, y          // our x,y coords
  let t1x, t2x, t1y, t2y // tension vectors
  let c1, c2, c3, c4     // cardinal points
  let st, t, i       // steps based on num. of segments

  // clone array so we don't change the original
  //
  _pts = pts.slice(0)

  // The algorithm require a previous and next point to the actual point array.
  // Check if we will draw closed or open curve.
  // If closed, copy end points to beginning and first points to end
  // If open, duplicate first points to befinning, end points to end
  if (isClosed) {
    _pts.unshift(pts[pts.length - 1])
    _pts.unshift(pts[pts.length - 2])
    _pts.unshift(pts[pts.length - 1])
    _pts.unshift(pts[pts.length - 2])
    _pts.push(pts[0])
    _pts.push(pts[1])
  }
  else {
    _pts.unshift(pts[1])   //copy 1. point and insert at beginning
    _pts.unshift(pts[0])
    _pts.push(pts[pts.length - 2]) //copy last point and append
    _pts.push(pts[pts.length - 1])
  }

  // ok, lets start..

  // 1. loop goes through point array
  // 2. loop goes through each segment between the 2 pts + 1e point before and after
  for (i=2; i < (_pts.length - 4); i+=2) {
    for (t=0; t <= numOfSegments; t++) {

      // calc tension vectors
      t1x = (_pts[i+2] - _pts[i-2]) * tension
      t2x = (_pts[i+4] - _pts[i]) * tension

      t1y = (_pts[i+3] - _pts[i-1]) * tension
      t2y = (_pts[i+5] - _pts[i+1]) * tension

      // calc step
      st = t / numOfSegments
      let st2 = st * st
      let st3 = st2 * st

      // calc cardinals
      c1 =   2 * st3  - 3 * st2 + 1
      c2 = -(2 * st3) + 3 * st2
      c3 =       st3  - 2 * st2 + st
      c4 =       st3  -     st2

      // calc x and y cords with common control vectors
      x = c1 * _pts[i]    + c2 * _pts[i+2] + c3 * t1x + c4 * t2x
      y = c1 * _pts[i+1]  + c2 * _pts[i+3] + c3 * t1y + c4 * t2y

      //store points in array
      res.push(x)
      res.push(y)
    }
  }

  return res
}
