<template lang="pug">
.config
  a.close(@click="close")
    b-icon(icon="close-circle-outline", size="is-large")

  .content.has-text-centered
    h1.title.is-size-3 Simulation Settings

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

  br
  .content
    .has-text-centered
      h2.title.is-size-3 Details about this simulation

    .columns.is-centered
      .column.is-two-thirds
        p.
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin eu tempus elit, in viverra lectus. Vivamus mi velit, vehicula id dui non, placerat viverra lacus. Integer sed justo dignissim, finibus elit porta, dapibus magna. Vestibulum tempus venenatis dignissim. Maecenas viverra tortor sodales, sodales est aliquet, dapibus lacus. Fusce vel ligula aliquet, semper justo vel, faucibus enim. Cras sit amet massa orci. Nulla gravida, nisi nec elementum auctor, ante elit cursus risus, vel semper metus velit vitae quam. Morbi fringilla ex tortor. Suspendisse id nunc eu metus pharetra faucibus.
        p.
          Phasellus eu mauris eget est lobortis venenatis. Vestibulum ac lacus sed libero feugiat viverra eu ac nisl. Nunc fermentum, sem vitae tempus vulputate, magna lectus luctus nisl, in malesuada nibh metus et quam. Etiam placerat tempus odio, eget commodo ligula mollis non. Aenean blandit sit amet risus eget condimentum. Duis eu auctor ante, a sodales mi. Morbi vulputate eleifend lacus non luctus. Nulla in semper nisl. Proin lacinia elementum mauris eu interdum. Maecenas in tempor augue. Nullam placerat libero id cursus dictum. Nullam ut faucibus quam.
        p.
          Ut pharetra purus ac orci porta, id auctor lorem imperdiet. Duis feugiat libero at mauris pharetra sollicitudin. Nulla vitae lacus eget nisl viverra mollis non at lorem. Aenean dignissim pulvinar ligula eleifend dictum. Quisque aliquam facilisis nisi in tristique. Donec diam diam, tempor vitae sagittis dictum, mattis in mi. Aenean aliquet a tellus finibus egestas. Ut ullamcorper enim ac purus rhoncus, ac fermentum quam sagittis.
        p.
          Donec a arcu sit amet sapien accumsan volutpat nec in justo. Aenean commodo urna eget pretium efficitur. Ut lacinia, eros ac mattis aliquam, nulla orci ultrices dolor, luctus ullamcorper risus arcu sit amet sapien. Ut vestibulum orci eget nisi luctus, ut condimentum erat vestibulum. Etiam eu turpis in est elementum consequat a non dolor. Nulla facilisi. Duis finibus arcu arcu, vel imperdiet leo sodales nec. Morbi rutrum diam consectetur tellus feugiat tristique. Donec condimentum ut leo id consequat. Etiam interdum ipsum iaculis ligula mattis mollis. Vestibulum vitae luctus elit, at feugiat lacus. Sed vel molestie neque. In dapibus, dolor eget volutpat venenatis, enim lacus facilisis erat, ut commodo nibh mi vel ligula.
        p.
          Phasellus commodo tellus metus, quis finibus purus lacinia ut. Integer egestas augue non dui aliquam, sed rutrum est semper. Sed vel purus id ipsum porttitor maximus quis at ante. Proin ac lectus at ipsum commodo sodales. Integer ut felis vitae eros faucibus consectetur in non lectus. Duis accumsan porttitor pharetra. Sed tempus nibh quis dolor vulputate, at porttitor metus semper. Proin condimentum placerat magna, laoreet pharetra turpis consequat vel. Praesent consectetur blandit tellus nec viverra.
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
