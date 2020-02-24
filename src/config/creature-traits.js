import chroma from 'chroma-js'
import _keyBy from 'lodash/keyBy'
import _mapValues from 'lodash/mapValues'
const traits = ['speed', 'sense_range', 'size', 'life_span', 'age']
const colors = chroma.scale('Set1').colors(8)
export default Object.freeze({
  traits
  , colors
  , hashed: Object.freeze(_mapValues(_keyBy(colors.map((c, i) => [c, i]), ([, i]) => traits[i]), ([c]) => c))
})
