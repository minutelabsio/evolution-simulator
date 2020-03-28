<template lang="pug">
.config
  a.close(@click="close")
    b-icon(icon="close-circle-outline", size="is-large")

  .content.has-text-centered
    h1.title.is-size-3 Simulation Settings

  .content
    .has-text-centered
      p More details about how this all works can be found in the <router-link :to="{ name: 'about', append }">about page</router-link>.

  .content
    .has-text-centered
      h2.title.is-size-4 First
    .columns
      .column.has-text-right-tablet.has-text-centered
        p We will start with this many creatures
        .is-inline-block
          NumberInput(v-model="creatureCount", label="Creatures to Start", :min="1", :change-rate="10", condensed)

      .column.has-text-left-tablet.has-text-centered
        p ...and each creature will start with these properties
        .is-inline-block
          .number-input-group
            NumberInput(v-model="speedValue", label="Speed", :min="0.05", :change-rate="10", :color="traitColors.speed")
            NumberInput(v-model="sizeValue", label="Size", :min="0.05", :change-rate="10", :color="traitColors.size")
            NumberInput(v-model="sense_rangeValue", label="Sense Range", :min="0.01", :change-rate="10", :color="traitColors.sense_range")
  .content
    .has-text-centered
      h2.title.is-size-4 Each generation
    .columns
      .column.has-text-right-tablet.has-text-centered
        p Creatures will start with this much energy
        .is-inline-block
          NumberInput(v-model="energy", label="Energy", :min="0", :change-rate="100", condensed, :color="traitColors.energy")
      .column.has-text-left-tablet.has-text-centered
        p ...and we will randomly place this much food
        .is-inline-block
          NumberInput(v-model="foodPerGeneration", label="Food", :min="0", :change-rate="10", condensed, :color="foodColor")

  .content
    .has-text-centered
      h2.title.is-size-4 When they reproduce
    .columns
      .column.has-text-right-tablet.has-text-centered
        p.
          The traits will mutate by randomly selecting a value
          (in a gaussian normal distribution) centered around their current trait value
          with a variance of...
      .column.has-text-left-tablet.has-text-centered
        .is-inline-block
          .number-input-group
            NumberInput(v-model="speedVariance", label="σ² Speed", :min="0.01", :max="10", :change-rate="1", :step="0.1", :color="traitColors.speed")
            NumberInput(v-model="sizeVariance", label="σ² Size", :min="0.01", :max="10", :change-rate="1", :step="0.1", :color="traitColors.size")
            NumberInput(v-model="sense_rangeVariance", label="σ² Sense", :min="0.01", :max="10", :change-rate="1", :step="0.1", :color="traitColors.sense_range")

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

  .content
    .has-text-centered
      h2.title.is-size-4 Finally
    .columns
      .column.has-text-centered
        b-field.is-inline-block
          .number-input-group
            NumberInput(v-model="maxGenerations", label="Days to run", :min="1", :change-rate="10")
            NumberInput(v-model="seed", label="Random Seed", :min="1", :change-rate="10")
        b-field
          b-button.button.is-primary.is-large(@click="run", :loading="isLoading") Evolve!
</template>

<script>
import { mapGetters } from 'vuex'
import traitColors from '@/config/trait-colors'
import sougy from '@/config/sougy-colors'
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
    traitColors
    , foodColor: sougy.green
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
.config
  padding: 5em 2em 2em
  margin-bottom: 0
  font-size: 18px
  p
    font-family: $family-sans-serif
  .close
    position: absolute
    top: 1rem
    right: 1rem
    color: lighten($grey, 20)
</style>
