<template lang="pug">
#app
  AppMessages
  router-view
</template>

<script>
// import PubSub from '@/lib/pubsub'
import AppMessages from '@/components/app-messages'
export default {
  name: 'App'
  , data: () => ({
  })
  , components: {
    AppMessages
  }
}
</script>

<style lang="sass" scoped>
#app
  position: fixed
  top: 0
  left: 0
  right: 0
  bottom: 0
</style>
