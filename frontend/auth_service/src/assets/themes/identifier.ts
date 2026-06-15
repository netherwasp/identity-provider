  import { definePreset } from '@primeuix/themes';
  import Material from '@primeuix/themes/material';

  const Identifier = definePreset(Material, {
    semantic: {
      primary: {
        50: '#f0fff4',
        100: '#dcffe8',
        200: '#b0ffd0',
        300: '#70ff9e',
        400: '#00ff41', // classic matrix green
        500: '#00cc33',
        600: '#009926',
        700: '#007a1e',
        800: '#005c16',
        900: '#003d0f',
        950: '#001f07',
      },
      colorScheme: {
        light: {
          surface: {
            0: '#0d0d0d',
            50: '#0d0d0d',
            100: '#111111',
            200: '#1a1a1a',
            300: '#222222',
            400: '#2a2a2a',
            500: '#333333',
            600: '#4a4a4a',
            700: '#666666',
            800: '#888888',
            900: '#aaaaaa',
            950: '#cccccc',
          },
          primary: {
            color: '{primary.400}',
            contrastColor: '#000000',
            hoverColor: '{primary.300}',
            activeColor: '{primary.200}',
          },
          text: {
            color: '#00ff41',
            hoverColor: '#70ff9e',
            mutedColor: '#009926',
            hoverMutedColor: '#00cc33',
          },
        },
        dark: {
          surface: {
            0: '#0d0d0d',
            50: '#0d0d0d',
            100: '#111111',
            200: '#1a1a1a',
            300: '#222222',
            400: '#2a2a2a',
            500: '#333333',
            600: '#4a4a4a',
            700: '#666666',
            800: '#888888',
            900: '#aaaaaa',
            950: '#cccccc',
          },
          primary: {
            color: '{primary.400}',
            contrastColor: '#000000',
            hoverColor: '{primary.300}',
            activeColor: '{primary.200}',
          },
          text: {
            color: '#00ff41',
            hoverColor: '#70ff9e',
            mutedColor: '#009926',
            hoverMutedColor: '#00cc33',
          },
        },
      },

      components: {
        button: {
          borderRadius: '0.2', 
          paddingX: '1.25rem',
          paddingY: '0.75rem',
        },
        inputtext: {
          borderRadius: '0',
        },
        card: {
          borderRadius: '2',
          // shadow: '0 0 20px rgba(0, 255, 65, 0.3)', // green glow
        },
        datatable: {
          headerBorderColor: '{primary.400}',
        },
      },
    },
  });

  export default Identifier;
