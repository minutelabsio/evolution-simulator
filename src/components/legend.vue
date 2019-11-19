<template lang="pug">
ul.legend
  li(v-for="item in data")
    span.color(:style="{ 'background-color': item.color }")
    span {{ item.name }}
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
.color
  display: inline-block
  width: 1em
  height: 1em
  margin-right: 1ex
</style>
