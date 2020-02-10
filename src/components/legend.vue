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
  flex-direction: row
  flex-wrap: wrap
  padding-left: 2em
  li
    display: flex
    align-items: center
    // margin-right: 1em
    margin-bottom: 0.5em
    width: 50%
    white-space: nowrap
    &:hover
      text-decoration: underline
.color
  display: inline-block
  width: 1em
  height: 1em
  margin-right: 1ex
  border-radius: 3px
</style>
