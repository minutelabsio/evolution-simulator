<template lang="pug">
.config
  a.close(@click="close")
    b-icon(icon="close-circle-outline", size="is-large")

  .content.has-text-centered
    h1.title.is-size-3 Simulation Settings

  .content
    .has-text-centered
      //- .preset-config
      //-   b-field
      //-     b-select(v-model="currentPreset")
      //-       option(v-for="p in presets", :value="p", :key="p") {{ p | startCase }}
      //-
      //-   .preset-config(v-if="currentPreset === 'home_remove'")
      //-     p Remove the blob's homes from three edges at the following generation:
      //-     b-field
      //-       NumberInput(v-model="presetOptions.step", label="Generation", :min="1", :step="1", :change-rate="10")
      //- br/
      p More details about how this all works can be found in the <router-link :to="{ name: 'about' }" append>about page</router-link>.

  .invasive(v-if="currentPreset === 'invasive_species'")
    .content
      .has-text-centered
        h2.title.is-size-4 Invasive Species (Orange Blobs)
      CreatureTemplateConfig(species="invasive_species")
  hr/
  .content
    .has-text-centered
      h2.title.is-size-4 Blob Creatures
    CreatureTemplateConfig(species="default")

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
          NumberInput(v-model="foodPerGeneration", label="Food", :min="0", :max="1000", :change-rate="10", condensed, :color="foodColor")


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
            NumberInput(v-model="maxGenerations", label="Days to run", :min="1", :max="2000", :change-rate="10")
            NumberInput(v-model="seed", label="Random Seed", :min="1", :change-rate="10")
        b-field
          b-button.button.is-primary.is-large(@click="run", :loading="isLoading") Evolve!
        p.has-text-grey (by natural selection)
</template>

<script>
import { mapGetters } from 'vuex'
import traitColors from '@/config/trait-colors'
import sougy from '@/config/sougy-colors'
import NumberInput from '@/components/inputs/number-input'
import CreatureTemplateConfig from '@/components/creature-template-config'

function storeParam(key, src, action){
  return {
    get(){ return this[src][key] }
    , set(v){ this.$store.dispatch(action, { [key]: v }) }
  }
}

function creatureProps(props, species){
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
    ret[k] = {
      get(){ return this.creatureTemplate(species)[k] }
      , set(v){ this.$store.dispatch('simulation/setCreatureTemplate', { [k]: v, species }) }
    }
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

    , presetOptions: {}
  })
  , components: {
    NumberInput
    , CreatureTemplateConfig
  }
  , mounted(){
    this.presetOptions = {...this.config.preset.options}
    this.$watch('presetOptions', () => {
      this.$store.dispatch('simulation/setConfig', { preset: {
        ...this.config.preset
        , options: this.presetOptions
      } })
    }, { deep: true })
  }
  , computed: {
    ...mapGetters('simulation', {
      isLoading: 'isLoading'
      , config: 'config'
      , creatureTemplate: 'creatureTemplate'
      , presets: 'presets'
    })

    , currentPreset: {
      get(){
        return this.config.preset.name
      }
      , set(name){
        this.$store.dispatch('simulation/setConfig', { preset: {
          ...this.config.preset
          , name
        } })
      }
    }
    , seed: storeParam('seed', 'config', 'simulation/setConfig')
    , maxGenerations: storeParam('max_generations', 'config', 'simulation/setConfig')
    , foodPerGeneration: storeParam('food_per_generation', 'config', 'simulation/setConfig')

    , creatureCount: {
      get(){ return this.$store.getters['simulation/creatureConfig']('default').count }
      , set(v){ this.$store.dispatch('simulation/setCreatureConfig', { count: v, species: 'default' }) }
    }
    , energy: {
      get(){ return this.$store.getters['simulation/creatureTemplate']('default').energy }
      , set(energy){ this.$store.dispatch('simulation/setCreatureTemplate', { energy, species: 'default' }) }
    }
    , ...creatureProps(['speed', 'size', 'sense_range', 'reach', 'life_span'], 'default')
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
  >>> .content
    margin-bottom: 2rem

  .close
    position: absolute
    top: 1rem
    right: 1rem
    color: lighten($grey, 20)

.preset-config
  display: flex
  flex-direction: column
  align-items: center
</style>
