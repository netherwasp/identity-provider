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
    danger: {
      50: '#fff0f0',
      100: '#ffdcdc',
      200: '#ffb0b0',
      300: '#ff7070',
      400: '#ff2424',
      500: '#e60000',
      600: '#b30000',
      700: '#8a0000',
      800: '#660000',
      900: '#400000',
      950: '#200000',
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
          ground: '{surface.50}', // surface-ground
          section: '{surface.50}', // surface-section
          card: '{surface.50}', // surface-card
          overlay: '{surface.200}', // surface-overlay
          border: '{surface.300}', // surface-border
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
      root: {
        borderRadius: '0',
        background: '{surface.100}',
        color: '{text.color}',
        placeholderColor: '{text.mutedColor}',
        borderColor: '{surface.300}',
        focusBorderColor: '{primary.color}',
        invalidColor: '{danger.400}',
      },
      focusRing: {
        width: '0',
        style: 'none',
        color: 'transparent',
        shadow: '0 0 12px color-mix(in srgb, var(--p-primary-color) 50%, transparent)',
      },
    } as any,

    inputgroup: {
      addon: {
        background: '{surface.100}',
        borderColor: '{surface.300}',
        color: '{text.mutedColor}',
      },
    } as any,

    iftalabel: {
      root: {
        color: '{text.mutedColor}',
        focusColor: '{primary.400}',
        invalidColor: '{danger.400}',
      },
    } as any,

    select: {
      root: {
        background: '{surface.100}',
        borderColor: '{surface.300}',
        color: '{text.color}',
        borderRadius: '0',
        hoverBorderColor: '{primary.color}',
        focusBorderColor: '{primary.color}',
        placeholderColor: '{text.mutedColor}',
        paddingX: '0.75rem',
        paddingY: '0.75rem',
        focusRing: {
          width: '0',
          style: 'none',
          color: 'transparent',
          shadow: '0 0 12px color-mix(in srgb, var(--p-primary-color) 50%, transparent)',
        },
      },

      dropdown: {
        width: '2.5rem',
        color: '{text.mutedColor}',
        hoverColor: '{primary.color}',
      },

      overlay: {
        background: '{surface.100}',
        borderColor: '{primary.color}',
        borderRadius: '0',
        color: '{text.color}',
        shadow: '0 0 30px color-mix(in srgb, var(--p-primary-color) 40%, transparent)',
      },

      option: {
        color: '{text.color}',
        background: '{surface.100}',
        focusBackground: '{surface.300}',
        focusColor: '{primary.300}',
        selectedBackground: '{primary.color}',
        selectedColor: '{primary.contrastColor}',
        selectedFocusBackground: '{primary.300}',
        selectedFocusColor: '{primary.contrastColor}',
        borderRadius: '0',
      },

      optionGroup: {
        background: '{surface.200}',
        color: '{text.mutedColor}',
      },
    } as any,

    card: {
      borderRadius: '2',
    } as any,

    datatable: {
      headerBorderColor: '{primary.400}',
    } as any,
  },
});

export default Identifier;
