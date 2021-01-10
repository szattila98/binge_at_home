<template lang="pug">
  video(ref="videoPlayer" class="video-js vjs-default-skin vjs-big-play-centered")
</template>

<script>
import videojs from 'video.js'
import hotkeys from 'videojs-hotkeys'
import persistVolume from 'videojs-persistvolume'

export default {
  name: 'Player',
  data () {
    return {
      player: null,
      options: {
        autoplay: false,
        controls: true,
        preload: 'auto',
        playbackRates: [0.5, 1.0, 2.0],
        sources: [{
          src: location.origin + '/api/stream?v=' + this.$route.params.name,
          type: `video/${this.$route.params.ext}`
        }],
        fluid: true,
        // poster: "/static/images/author.jpg", TODO poster on backend
        plugins: {
          hotkeys: {
            customClass: hotkeys
          },
          persistvolume: {
            customClass: persistVolume,
            namespace: 'binge-at-home'
          }
        }
      }
    }
  },
  mounted () {
    this.player = videojs(this.$refs.videoPlayer, this.options, function onPlayerReady () {
      console.log('onPlayerReady', this)
    })
  },
  beforeDestroy () {
    if (this.player) {
      this.player.dispose()
    }
  }
}
</script>
