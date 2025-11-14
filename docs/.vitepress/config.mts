import { defineConfig } from 'vitepress'
import('vite').UserConfig

// https://vitepress.dev/reference/site-config
export default defineConfig({
  vite: {
    server: {
      allowedHosts: true
    },
  },
  title: "Wavewall",
  description: "Wavewall User Documentation",
  base: "/wavewall/",
  themeConfig: {
    nav: [
   { text: 'Home', link: '/' },
    ],

    sidebar: [
      { text: 'Introduction', link: '/introduction'},
      {
        text: 'Wavewall',
        items: [
          { text: 'Output', link: '/wavewall/output' },
          { text: 'Generation', link: '/wavewall/generation' },
        ],
      },
      {
        text: 'Tileset',
        items: [
          { text: 'Info', link: '/tileset/info' },
          { text: 'Selection', link: '/tileset/selection' },
          { text: 'Pseudotiles', link: '/tileset/pseudotiles' },
          { text: 'Recipes', link: 'tileset/recipes' },
          { text: 'Colorizer', link: 'tileset/colorizer' },
        ],
      },
      {
        text: 'Commands',
        items: [
          { text: 'Template', link: '/commands/template' },
          { text: 'Colors', link: '/commands/colors' }
        ],
      },
      {
        text: 'Lua',
        items: [
          { text: 'Guide', link: '/lua/guide' },
          { text: 'Provided Functions', link: '/lua/provided_functions' }
        ],
      },
      {
        text: 'UserData',
        items: [
          { text: 'PixelInfo', link: '/userdata/pixelinfo' },
          { text: 'ColorInfo', link: '/userdata/colorinfo' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/rohrsben/wavewall' }
    ]
  }
})
