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
          ground: '#333333',
          card: '#333333',
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
          0: '#1a1a1e',
          50: '#1e1e1e',
          100: '#242436',
          200: '#2a2a40',
          300: '#32324a',
          400: '#3d3d55',
          500: '#4a4a62',
          600: '#606070',
          700: '#808090',
          800: '#a0a0b0',
          900: '#c0c0cc',
          950: '#e0e0e8',
          ground: '{surface.50}',    // surface-ground
          section: '{surface.50}',  // surface-section
          card: '{surface.50}',    // surface-card
          overlay: '{surface.200}', // surface-overlay
          border: '{surface.300}',  // surface-border
          hover: '{surface.400}',
        },
        primary: {
          color: '{primary.400}',
          contrastColor: '#000000',
          hoverColor: '{primary.300}',
          activeColor: '{primary.200}',
        },
        text: {
          color: '#70ff9e', // softer green instead of harsh #00ff41
          hoverColor: '#5abb6e',
          mutedColor: '#00cc33',
          hoverMutedColor: '#00ff41',
        },
      },
    },
  },

  components: {
    button: {
      borderRadius: '0.2rem',
      paddingX: '1.25rem',
      paddingY: '0.75rem',
    } as any,
    inputtext: {
      borderRadius: '0',
    } as any,
    card: {
      borderRadius: '2',
      // shadow: '0 0 20px rgba(0, 255, 65, 0.3)', // green glow
    } as any,
    datatable: {
      headerBorderColor: '{primary.400}',
    } as any,
  },
});

export default Identifier;
