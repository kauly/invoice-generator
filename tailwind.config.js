module.exports = {
  mode: "jit",
  content: ["src/**/*.rs"],
  theme: {
    extend: {
      aspectRatio: {
        "4/3": "4 / 3",
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
