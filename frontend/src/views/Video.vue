<template lang="pug">
  .container.row.col-10.center
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
  left: 50%;
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
}
</style>
