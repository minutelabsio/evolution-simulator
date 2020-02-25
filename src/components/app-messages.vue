<template lang="pug">
.app-messages
  InfoMessage(v-for="alert in infos", :alert="alert", :key="alert.id")
  ErrorMessage(v-for="alert in errors", :alert="alert", :key="alert.id")
</template>

<script>
import { mapGetters } from 'vuex'
import InfoMessage from '@/components/alerts/info-message'
import ErrorMessage from '@/components/alerts/error-message'

export default {
  name: 'AppMessages'
  , props: {
  }
  , data: () => ({
  })
  , components: {
    InfoMessage
    , ErrorMessage
  }
  , computed: {
    ...mapGetters([
      'errors'
      , 'infos'
    ])
  }
  , methods: {
  }
}
</script>
