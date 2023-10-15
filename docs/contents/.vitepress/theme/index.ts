import { inBrowser, useData } from 'vitepress'
import DefaultTheme from 'vitepress/theme'

import "./vars.styl"
import "./rainbow.styl"

import { watchEffect } from "vue"
//@ts-ignore
import Layout from "./Layout.vue"

export default {
  ...DefaultTheme,
  Layout,
  setup() {
    const { lang } = useData()
    watchEffect(() => {
      if (inBrowser) {
        document.cookie = `nf_lang=${lang.value}; expires=Mon, 1 Jan 2099 00:00:00 UTC; path=/`
      }
    })
  }
}