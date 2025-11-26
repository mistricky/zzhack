module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        page: "var(--color-page-bg)",
        card: "var(--color-card-bg)",
        border: "var(--color-border)",
        text: "var(--color-text)",
        hover: "var(--color-hover)",
        halo: "var(--color-halo)",
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
