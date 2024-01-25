export default defineNuxtPlugin(() => {
  if (process.client) {
    window.addEventListener("keydown", (event) => {
      if ((event.ctrlKey || event.metaKey) && event.key === "r") {
        event.preventDefault();
      }
      if (event.key === "F5") {
        event.preventDefault();
      }
    });
  }
});
