<template lang="pug">
  .container.row.col-8.center
    Player
</template>

<script>
import Player from '@/components/Player'

export default {
  name: 'Video',
  components: { Player }
}
</script>

<style lang="scss" scoped>
.center {
  margin: 0 auto;
  width: auto;
}
</style>
