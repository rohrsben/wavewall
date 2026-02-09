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
      {
        text: 'Configuration',
        items: [
          { text: 'Output', link: '/configuration/output' },
          { text: 'Tileset', link: '/configuration/tileset' },
          { text: 'Colorizer', link: '/configuration/colorizer' }
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
          { text: 'PlacerInfo', link: '/userdata/placerinfo' },
        ],
      },
      {
        text: 'Other',
        items: [
          { text: 'Pseudotiles', link: '/other/pseudotiles' },
          { text: 'Recipes', link: '/other/recipes' }
        ],
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/rohrsben/wavewall' }
    ]
  }
})
