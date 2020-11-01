<template lang="pug">
  table.table.table-hover
    thead
      tr
        th Video Name
        th File Size
        th Creation date
        th Last Viewed date
    tbody
      tr(v-for="video in this.data" @click="view(video.fileName)").videoRow
        td {{ video.fileName }}
        td {{ video.size | prettyBytes }}
        td {{ video.created | moment('DD/MM/YYYY') }}
        td {{ video.lastAccessed | moment('DD/MM/YYYY')}}
</template>

<script>
export default {
  name: 'VideoList',
  props: {
    data: {
      type: Array,
      required: true
    }
  },
  methods: {
    view (name) {
      this.$router.push({
        name: 'Video',
        params: { name: name }
      })
    }
  }
}
</script>

<style lang="scss" scoped>
.videoRow {
  cursor: pointer;
}
</style>
