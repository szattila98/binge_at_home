// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["@nuxtjs/eslint-module", "nuxt-typed-router"],
  typescript: {
    typeCheck: true,
  },
});
