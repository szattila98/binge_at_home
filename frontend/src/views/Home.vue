<template lang="pug">
  .container.row.col-12
    VideoList(:data="this.videos")
</template>

<script>
import VideoList from '@/components/VideoList'
import { mapActions, mapState } from 'vuex'

export default {
  name: 'Home',
  components: {
    VideoList
  },
  computed: {
    ...mapState(['videos'])
  },
  created () {
    this.getVideos()
  },
  methods: {
    ...mapActions(['getVideos'])
  }
}
</script>
