import chroma from 'chroma-js'
import sougy from '@/config/sougy-colors'

export const creatureColors = {
  'default': chroma(sougy.blue).desaturate(0.5).num()
  , 'orange': chroma(sougy.orange).num()
  , 'pink': chroma(sougy.pink).num()
}
