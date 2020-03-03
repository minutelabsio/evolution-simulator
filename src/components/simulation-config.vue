<template lang="pug">
.config.content
  a.close(@click="close")
    b-icon(icon="close-circle-outline", size="is-large")

  h4.title.is-size-3 Simulation Settings

  p Creatures will start each generation with this much energy
  b-field(grouped)
    NumberInput(v-model="energy", label="Energy", :min="0", :change-rate="100", condensed, :color="energyColor")

  p Creatures will start the simulation with these properties
  b-field
    NumberInput(v-model="speedValue", label="Speed", :min="0.01", :change-rate="10", condensed, :color="creatureColors.speed")
  b-field
    NumberInput(v-model="sizeValue", label="Size", :min="0.01", :change-rate="10")
  b-field
    NumberInput(v-model="sense_rangeValue", label="Sense Range", :min="0.01", :change-rate="10", condensed)
  b-field
    NumberInput(v-model="speedValue", label="Speed", :min="0.01", :change-rate="10")


  //- b-field(grouped)
  //-   b-field(label="Reach")
  //-     b-input(v-model="reachValue", type="number", min="0", step="any")
  //-   b-field(label="Reach mutation variance")
  //-     b-input(v-model="reachVariance", type="number", min="0", step="any")
  //- b-field(grouped)
  //-   b-field(label="Avg Lifespan")
  //-     b-input(v-model="life_spanValue", type="number", min="0", step="any")
  //-   b-field(label="Avg Lifespan mutation variance")
  //-     b-input(v-model="life_spanVariance", type="number", min="0", step="any")

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
import CreatureTraits from '@/config/creature-traits'
import NumberInput from '@/components/inputs/number-input'

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
    creatureColors: CreatureTraits.hashed
    , energyColor: CreatureTraits.energyColor
  })
  , components: {
    NumberInput
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
.config.content
  padding: 5em 2em 2em
  margin-bottom: 0
  p
    font-family: $family-sans-serif
  .close
    position: absolute
    top: 1em
    right: 1em
    color: lighten($grey, 20)
</style>
