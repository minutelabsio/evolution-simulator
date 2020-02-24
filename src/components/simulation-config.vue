<template lang="pug">
.config
  a.close(@click="close")
    b-icon(icon="close-circle-outline", size="is-large")
  h4.title.is-size-5 Initial Creature Properties
  b-field(grouped)
    b-field(label="Energy")
      b-input(v-model="energy", type="number", min="0", step="any")
  b-field(grouped)
    b-field(label="Speed")
      b-input(v-model="speedValue", type="number", min="0", step="any")
    b-field(label="Speed mutation variance")
      b-input(v-model="speedVariance", type="number", min="0", step="any")
  b-field(grouped)
    b-field(label="Size")
      b-input(v-model="sizeValue", type="number", min="0", step="any")
    b-field(label="Size mutation variance")
      b-input(v-model="sizeVariance", type="number", min="0", step="any")
  b-field(grouped)
    b-field(label="Sense")
      b-input(v-model="sense_rangeValue", type="number", min="0", step="any")
    b-field(label="Sense mutation variance")
      b-input(v-model="sense_rangeVariance", type="number", min="0", step="any")
  b-field(grouped)
    b-field(label="Reach")
      b-input(v-model="reachValue", type="number", min="0", step="any")
    b-field(label="Reach mutation variance")
      b-input(v-model="reachVariance", type="number", min="0", step="any")
  b-field(grouped)
    b-field(label="Avg Lifespan")
      b-input(v-model="life_spanValue", type="number", min="0", step="any")
    b-field(label="Avg Lifespan mutation variance")
      b-input(v-model="life_spanVariance", type="number", min="0", step="any")

  b-field(grouped)
    b-field(label="Number of Creatures")
      b-input(v-model="creatureCount", type="number", min="1", step="1")
    b-field(label="Food Per Generation")
      b-input(v-model="foodPerGeneration", type="number", min="0", step="1")
  b-field(grouped)
    b-field(label="Random Seed")
      b-input(v-model="seed", type="number", min="0", step="1")
    b-field(label="Max Generations")
      b-input(v-model="maxGenerations", type="number", min="0", step="1")
  b-field
    b-button.button.is-primary.is-large(@click="run", :loading="isLoading") Run!
</template>

<script>
import { mapGetters } from 'vuex'

function storeParam(key, src, action){
  return {
    get(){ return this[src][key] }
    , set(v){ this.$store.dispatch(action, { [key]: v }) }
  }
}

function creatureProps(props){
  let ret = {}
  for (let k of props){
    ret[k + 'Value'] = {
      get(){ return this[k][0] }
      , set(v){ this[k] = [v, this[k + 'Variance']] }
    }
    ret[k + 'Variance'] = {
      get(){ return this[k][1] }
      , set(v){ this[k] = [this[k + 'Value'], v] }
    }
    ret[k] = storeParam(k, 'creatureTemplate', 'simulation/setCreatureTemplate')
  }
  return ret
}

export default {
  name: 'SimulationConfig'
  , props: {
  }
  , data: () => ({
  })
  , components: {

  }
  , mounted(){
  }
  , computed: {
    ...mapGetters('simulation', {
      isLoading: 'isLoading'
      , config: 'config'
      , creatureTemplate: 'creatureTemplate'
    })

    , seed: storeParam('seed', 'config', 'simulation/setConfig')
    , maxGenerations: storeParam('max_generations', 'config', 'simulation/setConfig')
    , foodPerGeneration: storeParam('food_per_generation', 'config', 'simulation/setConfig')

    , creatureCount: {
      get(){ return this.$store.getters['simulation/creatureCount'] }
      , set(v){ this.$store.dispatch('simulation/setCreatureCount', v) }
    }
    , energy: storeParam('energy', 'creatureTemplate', 'simulation/setCreatureTemplate')
    , ...creatureProps(['speed', 'size', 'sense_range', 'reach', 'life_span'])
  }
  , watch: {
  }
  , methods: {
    run(){
      this.$store.dispatch('simulation/run').then(() => {
        this.close()
      })
    }
    , close(){
      if ( !this.$route.query.cfg ){ return }
      this.$router.replace({ params: this.$route.params, query: {...this.$route.query, cfg: ""} })
    }
  }
}
</script>

<style lang="sass" scoped>
.config
  padding: 2em
  .close
    position: absolute
    top: 1em
    right: 1em
    color: lighten($grey, 20)
</style>
