<template lang="pug">
.creature-template-config
  .content
    .columns
      .column.has-text-right-tablet.has-text-centered
        p We will start with this many creatures
      .column.has-text-left-tablet.has-text-centered
        .is-inline-block
          NumberInput(v-model="creatureCount", label="Num. Blobs at Start", :min="1", :max="1000", :change-rate="10", condensed)

    .columns
      .column.has-text-right-tablet.has-text-centered
        p and each creature will start with these properties
      .column.has-text-left-tablet.has-text-centered
        .is-inline-block
          .number-input-group
            NumberInput(v-model="speedValue", label="Speed", :min="0.05", :change-rate="10", :color="traitColors.speed")
            NumberInput(v-model="sizeValue", label="Size", :min="0.05", :change-rate="10", :color="traitColors.size")
            NumberInput(v-model="sense_rangeValue", label="Sense Range", :min="0.01", :change-rate="10", :color="traitColors.sense_range")
        p.warning
          b-icon(icon="alert-box-outline")
          span Caution: setting these values too low without also lowering the energy will result in a very long calculation.
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
            NumberInput(v-model="speedVariance", label="σ² Speed", :min="0", :max="10", :change-rate="1", :step="0.1", :color="traitColors.speed")
            NumberInput(v-model="sizeVariance", label="σ² Size", :min="0", :max="10", :change-rate="1", :step="0.1", :color="traitColors.size")
            NumberInput(v-model="sense_rangeVariance", label="σ² Sense", :min="0", :max="10", :change-rate="1", :step="0.1", :color="traitColors.sense_range")
</template>

<script>
import { mapGetters } from 'vuex'
import traitColors from '@/config/trait-colors'
import NumberInput from '@/components/inputs/number-input'

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
    ret[k] = {
      get(){ return this.creatureTemplate(this.species)[k] }
      , set(v){ this.$store.dispatch('simulation/setCreatureTemplate', { [k]: v, species: this.species }) }
    }
  }
  return ret
}

export default {
  name: 'CreatureTemplateConfig'
  , props: {
    species: {
      type: String
      , default: 'default'
    }
  }
  , data: () => ({
    traitColors
  })
  , components: {
    NumberInput
  }
  , mounted(){
  }
  , computed: {
    ...mapGetters('simulation', {
      config: 'config'
      , creatureTemplate: 'creatureTemplate'
    })

    , creatureCount: {
      get(){ return this.$store.getters['simulation/creatureConfig'](this.species).count }
      , set(v){ this.$store.dispatch('simulation/setCreatureConfig', { count: v, species: this.species }) }
    }
    , energy: {
      get(){ return this.$store.getters['simulation/creatureTemplate'](this.species).energy }
      , set(energy){ this.$store.dispatch('simulation/setCreatureTemplate', { energy, species: this.species }) }
    }
    , ...creatureProps(['speed', 'size', 'sense_range', 'reach', 'life_span'])
  }
  , watch: {
  }
  , methods: {
  }
}
</script>

<style lang="sass" scoped>
.creature-template-config
  font-size: 18px
  p
    font-family: $family-sans-serif
.warning
  color: darken($grey-light, 20)
  max-width: 400px
</style>
