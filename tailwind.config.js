/** @type {import('tailwindcss').Config} */
const colors = {
  "line-divider": "#F0F0F0",
  gray: {
    10: "#FCFCFC",
    30: "#F7F7F7",
    100: "#E5E5E5",
    150: "#D9D9D9",
    400: "#999999",
    500: "#808080",
    600: "#666666",
    750: "#404040",
    900: "#1A1A1A"
  }
};

module.exports = {
  content: ["**/*.rs"],
  theme: {
    extend: {
        colors: {
        "default": colors.gray[900],
        "default-subdued": colors.gray[750],
        "body": colors.gray[600],
        "object-default": colors.gray[400],
        "object-highlight-bg-emphasized": colors.gray[100],
        "material-base": colors.gray[30],
        ...colors
      },
      fontSize: {
        xs: ['9px', '11px'],
        sm: ['11px', '16px'],
        base: ['13px', '20px'],
        lg: ['15px', '24px'],
        xl: ['17px', '28px'],
        "2xl": ['19px', '32px'],
        "3xl": ['23px', '36px'],
      },
      fontFamily: {
        sans: ['"ui-sans-serif", "system-ui", "-apple-system", "BlinkMacSystemFont", "Segoe UI", "Roboto", "Helvetica Neue", "Arial", "Noto Sans", "sans-serif", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"', {
          fontFeatureSettings: '"ss01", "ss02"',
          fontVariationSettings: '"opsz" 32'
        }],
      },
    },
  },
  plugins: [],
}
