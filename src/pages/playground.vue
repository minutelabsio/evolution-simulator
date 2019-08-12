<template lang="pug">
.playground
  .container
    .section
      .content
        | Test
</template>

<script>
import createWorker from '@/workers/world'
const worker = createWorker()

export default {
  name: 'Playground'
  , props: {
  }
  , components: {
  }
  , data: () => ({
  })
  , mounted(){
    worker.test().then( result => {
      console.log('wasm said ', result)
    })
  }
}
</script>

<style lang="sass" scoped>
</style>
