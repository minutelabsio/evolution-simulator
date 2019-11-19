<template lang="pug">
ul.legend
  li.clickable(v-for="(item, index) in data", @click="$emit('select', { item, index })")
    span.color(:style="{ 'background-color': item.color }")
    span {{ item.name | startCase }}
</template>

<script>
export default {
  name: 'Legend'
  , props: {
    data: Array
  }
}
</script>

<style lang="sass" scoped>
.legend
  display: flex
  li
    display: flex
    align-items: center
    margin-right: 1em
    &:hover
      text-decoration: underline
.color
  display: inline-block
  width: 1em
  height: 1em
  margin-right: 1ex
</style>
